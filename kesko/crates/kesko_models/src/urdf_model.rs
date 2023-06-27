use std::f32::consts::FRAC_2_PI;

use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::texture,
    transform,
    utils::{BoxedFuture, HashMap},
};
use serde::Deserialize;
use urdf_rs;

use kesko_core::interaction::groups::GroupDynamic;
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    collider::{ColliderPhysicalProperties, ColliderShape},
    force::Force,
    gravity::GravityScale,
    joint::fixed::FixedJoint,
    rigid_body::RigidBody,
};

#[derive(Debug, TypeUuid)]
#[uuid = "3c1bde14-cb57-4704-9d8f-6fbeec5500df"]
pub struct UrdfAsset {
    pub robot: urdf_rs::Robot,
}

#[derive(Component, Default)]
pub struct IsUrdfSpawned(pub bool);

#[derive(Default, Bundle)]
pub struct UrdfBundle {
    pub urdf_asset: Handle<UrdfAsset>,
    pub transform: Transform,
    pub is_urdf_spawned: IsUrdfSpawned,
}

pub struct UrdfModel;
impl UrdfModel {
    pub fn spawn(
        commands: &mut Commands,
        asset_path: impl Into<AssetPath<'static>>,
        transform: Transform,
        asset_server: &Res<AssetServer>,
    ) {
        let urdf_asset: Handle<UrdfAsset> = asset_server.load(asset_path);

        println!("urdf_asset: {:?}", urdf_asset);
        // Convert from ROS coordinate frame is Z up, Y left, X forward
        //   to Bevy coordinate frame: Y up, X right, Z backward
        let transform = transform
            * Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
            * Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

        commands.spawn((
            UrdfBundle {
                urdf_asset,
                transform,
                ..Default::default()
            },
            RigidBody::Fixed,
        ));
        // wait for asset to load

        // commands.spawn((
        //     SceneBundle {
        //         scene: urdf_asset,
        //         transform,
        //         ..Default::default()
        //     },
        //     RigidBody::Dynamic,
        //     ColliderShape::Sphere { radius: 0.2 },
        //     InteractiveBundle::<GroupDynamic>::default(),
        //     Force::default(),
        //     ColliderPhysicalProperties {
        //         restitution: 0.7,
        //         ..default()
        //     },
        //     GravityScale::default(),
        // ));
    }
}

fn urdf_pose_to_transform(pose: &urdf_rs::Pose) -> Transform {
    let xyz = Vec3::new(
        pose.xyz.0[0] as f32,
        pose.xyz.0[1] as f32,
        pose.xyz.0[2] as f32,
    );
    let rpy = Vec3::new(
        pose.rpy.0[0] as f32,
        pose.rpy.0[1] as f32,
        pose.rpy.0[2] as f32,
    );
    let rotation = Quat::from_euler(EulerRot::ZYX, rpy.z, rpy.y, rpy.x);
    let transform = Transform::from_translation(xyz).with_rotation(rotation);
    transform
}

pub fn convert_urdf_to_components(
    mut commands: Commands,
    mut urdf_models: Query<(Entity, &Handle<UrdfAsset>, &mut IsUrdfSpawned)>,
    urdf_assets: Res<Assets<UrdfAsset>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (root_entity, urdf_asset, mut is_urdf_spawned) in urdf_models.iter_mut() {
        if !is_urdf_spawned.0 {
            if let Some(urdf_asset) = urdf_assets.get(urdf_asset) {
                // print the parts of the robot
                for link in &urdf_asset.robot.links {
                    println!("link: {}", link.name);
                }
                for joint in &urdf_asset.robot.joints {
                    println!("joint: {}", joint.name);
                }
                for material in &urdf_asset.robot.materials {
                    println!("material: {:?}", material);
                }

                let urdf_robot = urdf_asset.robot.clone();
                // let link_joint_map = k::urdf::link_to_joint_map(&urdf_robot);
                let mut link_entity_map: HashMap<String, Entity> = HashMap::default();

                for link in &urdf_robot.links {
                    let part = commands
                        .spawn((
                            Name::new(link.name.clone()),
                            RigidBody::Dynamic,
                            TransformBundle::default(),
                            VisibilityBundle::default(),
                        ))
                        .id();

                    link_entity_map.insert(link.name.clone(), part);

                    for visual in link.visual.iter() {
                        println!("\n\nvisual: {:?}", visual);
                        let transform = urdf_pose_to_transform(&visual.origin);
                        let visual_entity = commands
                            .spawn((Name::new(
                                visual.name.clone().unwrap_or("unnamed_visual".to_string()),
                            ),))
                            .id();
                        commands.entity(part).add_child(visual_entity);

                        // Material with the same name in the root of the urdf
                        let top_urdf_material = urdf_asset
                            .robot
                            .materials
                            .iter()
                            .find(|m| m.name == visual.material.as_ref().unwrap().name);
                        let urdf_material = visual.material.as_ref();
                        let texture = urdf_material
                            .and_then(|m| m.texture.as_ref())
                            .or(top_urdf_material.and_then(|m| m.texture.as_ref()));
                        let color = urdf_material
                            .and_then(|m| m.color.as_ref())
                            .or(top_urdf_material.and_then(|m| m.color.as_ref()));

                        let material = if let Some(texture) = texture {
                            println!("texture: {:?}", texture);
                            let texture_path = asset_server.load(texture.filename.as_str());
                            materials.add(texture_path.into())
                        } else if let Some(color) = color {
                            println!("color: {:?}", color);
                            materials.add(
                                Color::rgba(
                                    color.rgba[0] as f32,
                                    color.rgba[1] as f32,
                                    color.rgba[2] as f32,
                                    color.rgba[3] as f32,
                                )
                                .into(),
                            )
                        } else {
                            materials.add(Color::DARK_GRAY.into())
                        };

                        print!("processing link: {}", link.name);
                        match visual.geometry {
                            urdf_rs::Geometry::Sphere { radius } => {
                                println!("sphere radius: {}", radius);
                                commands.entity(visual_entity).insert(PbrBundle {
                                    material: material.clone(),
                                    mesh: meshes.add(
                                        shape::Icosphere {
                                            radius: radius as f32,
                                            subdivisions: 5,
                                        }
                                        .try_into()
                                        .unwrap(),
                                    ),
                                    transform,
                                    ..default()
                                });
                            }
                            urdf_rs::Geometry::Box { size } => {
                                println!("box size: {:?}", size);
                                commands.entity(visual_entity).insert(PbrBundle {
                                    material: material.clone(),
                                    mesh: meshes.add(
                                        shape::Box {
                                            min_x: -size.0[0] as f32 / 2.0,
                                            min_y: -size.0[1] as f32 / 2.0,
                                            min_z: -size.0[2] as f32 / 2.0,
                                            max_x: size.0[0] as f32 / 2.0,
                                            max_y: size.0[1] as f32 / 2.0,
                                            max_z: size.0[2] as f32 / 2.0,
                                        }
                                        .try_into()
                                        .unwrap(),
                                    ),
                                    transform,
                                    ..default()
                                });
                            }
                            urdf_rs::Geometry::Cylinder { radius, length } => {
                                println!("cylinder radius: {}, length: {}", radius, length);
                            }
                            urdf_rs::Geometry::Capsule { radius, length } => {
                                println!("capsule radius: {}, length: {}", radius, length);
                            }
                            urdf_rs::Geometry::Mesh {
                                ref filename,
                                scale,
                            } => {
                                // replace path prefix
                                let filename = filename.replace("package://crane_x7_description", "/home/azazdeaz/repos/art-e-fact/wizard_separate_tests/gen/crane_x7_test_project/src/crane_x7_description");
                                let filename = filename.replace("package://sciurus17_description", "/home/azazdeaz/repos/art-e-fact/wizard_separate_tests/gen/crane_x7_test_project/src/sciurus17_ros/sciurus17_description");
                                println!("mesh filename: {}, scale: {:?}", filename, scale);

                                commands.entity(visual_entity).insert(PbrBundle {
                                    material: material.clone(),
                                    mesh: asset_server.load(filename.as_str()),
                                    transform,
                                    ..default()
                                });
                            }
                        };
                    }
                }

                // Find the root link
                let links = link_entity_map
                    .keys()
                    .filter(|name| {
                        !urdf_robot
                            .joints
                            .iter()
                            .find(|joint| joint.child.link == **name)
                            .is_some()
                    })
                    .collect::<Vec<_>>();
                if links.len() != 1 {
                    panic!("expected exactly one root link, found {}", links.len());
                }
                let Some(root_link) = link_entity_map.get(links[0]) else {
                    panic!("Can't find entity for root link '{}'", links[0]);
                };
                // commands.entity(root_entity).add_child(*root_link);
                commands
                    .entity(*root_link)
                    .insert(FixedJoint::attach_to(root_entity));

                for joint in &urdf_robot.joints {
                    let Some(parent) = link_entity_map.get(&joint.parent.link) else {
                        warn!("parent of joint '{}' not found: {}", joint.name, joint.parent.link);
                        continue;
                    };
                    let Some(child) = link_entity_map.get(&joint.child.link) else {
                        warn!("child of joint '{}' not found: {}", joint.name, joint.child.link);
                        continue;
                    };
                    println!("Join {} -> {}", joint.parent.link, joint.child.link);
                    match joint.joint_type {
                        // urdf_rs::JointType::Fixed => {
                        _ => {
                            println!("Add fixed joint {} -> {:?}", joint.name, joint.origin);
                            commands.entity(*child).insert(
                                FixedJoint::attach_to(*parent)
                                    .with_parent_anchor(urdf_pose_to_transform(&joint.origin)),
                            );
                        } // urdf_rs::JointType::Revolute => {
                          //     println!("revolute joint: {}", joint.name);
                          //     let parent = parent.unwrap();
                          //     let child = child.unwrap();
                          //     commands.push_children(parent.0, &[child.0]);
                          // }
                          // urdf_rs::JointType::Continuous => {
                          //     println!("continuous joint: {}", joint.name);
                          //     let parent = parent.unwrap();
                          //     let child = child.unwrap();
                          //     commands.push_children(parent.0, &[child.0]);
                          // }
                          // urdf_rs::JointType::Prismatic => {
                          //     println!("prismatic joint: {}", joint.name);
                          //     let parent = parent.unwrap();
                          //     let child = child.unwrap();
                          //     commands.push_children(parent.0, &[child.0]);
                          // }
                          // urdf_rs::JointType::Planar => {
                          //     println!("planar joint: {}", joint.name);
                          //     let parent = parent.unwrap();
                          //     let child = child.unwrap();
                          //     commands.push_children(parent.0, &[child.0]);
                          // }
                          // urdf_rs::JointType::Floating => {
                          //     println!("floating joint: {}", joint.name);
                          //     let parent = parent.unwrap();
                          //     let child = child.unwrap();
                          //     commands.push_children(parent.0, &[child.0]);
                          // }
                          // urdf_rs::JointType::Unknown => {
                          //     println!("unknown joint: {}", joint.name);
                          //     let parent = parent.unwrap();
                          //     let child = child.unwrap();
                          //     commands.push_children(parent.0, &[child.0]);
                          // }
                    }
                }
                is_urdf_spawned.0 = true;
            }
        }
    }
}

#[derive(Default)]
pub struct UrdfAssetLoader;

impl AssetLoader for UrdfAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        println!("FINISH URDF ASSET LOADER!!!!!!!!!!!!!!!!!!!1");
        Box::pin(async move {
            let robot = urdf_rs::read_from_string(std::str::from_utf8(bytes).unwrap()).unwrap();
            let asset = UrdfAsset { robot };
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["urdf"]
    }
}

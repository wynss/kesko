use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
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

        commands.spawn((UrdfBundle {
            urdf_asset,
            transform,
            ..Default::default()
        },));
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
    // let iso3 = k::urdf::isometry_from::<f32>(&visual.origin);
    // let transform = Transform::from_xyz(iso3.translation.vector.x, iso3.translation.vector.y, iso3.translation.vector.z)
    //     .with_rotation(Quat::new((iso3.rotation.vector().into()));
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
    Transform::from_translation(xyz).with_rotation(rotation)
}

pub fn convert_urdf_to_components(
    mut commands: Commands,
    mut urdf_models: Query<(Entity, &Handle<UrdfAsset>, &mut IsUrdfSpawned)>,
    asset_server: Res<Assets<UrdfAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, urdf_asset, mut is_urdf_spawned) in urdf_models.iter_mut() {
        if !is_urdf_spawned.0 {
            if let Some(urdf_asset) = asset_server.get(urdf_asset) {
                // print the parts of the robot
                for link in &urdf_asset.robot.links {
                    println!("link: {}", link.name);
                }
                for joint in &urdf_asset.robot.joints {
                    println!("joint: {}", joint.name);
                }
                for material in &urdf_asset.robot.materials {
                    println!("material: {}", material.name);
                }

                let urdf_robot = urdf_asset.robot.clone();
                // let link_joint_map = k::urdf::link_to_joint_map(&urdf_robot);
                let mut link_entity_map: HashMap<String, Entity> = HashMap::default();

                for link in &urdf_robot.links {
                    for visual in link.visual.iter() {
                        let transform = urdf_pose_to_transform(&visual.origin);

                        
                        let part = match visual.geometry {
                            urdf_rs::Geometry::Sphere { radius } => {
                                println!("sphere radius: {}", radius);
                                Some(
                                    commands
                                        .spawn((PbrBundle {
                                            // material,
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
                                        },))
                                        .id(),
                                )
                            }
                            urdf_rs::Geometry::Box { size } => {
                                println!("box size: {:?}", size);
                                Some(
                                    commands
                                        .spawn((PbrBundle {
                                            // material,
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
                                        },))
                                        .id(),
                                )
                            }
                            urdf_rs::Geometry::Cylinder { radius, length } => {
                                println!("cylinder radius: {}, length: {}", radius, length);
                                None
                            }
                            urdf_rs::Geometry::Capsule { radius, length } => {
                                println!("capsule radius: {}, length: {}", radius, length);
                                None
                            }
                            urdf_rs::Geometry::Mesh {
                                ref filename,
                                scale,
                            } => {
                                println!("mesh filename: {}, scale: {:?}", filename, scale);
                                None
                            }
                        };
                        if let Some(part) = part {
                            link_entity_map.insert(link.name.clone(), part);
                        }
                    }

                    for joint in &urdf_robot.joints {
                        let Some(parent) = link_entity_map.get(&joint.parent.link) else {
                            warn!("parent of joint '{}' not found: {}", joint.name, joint.parent.link);
                            continue;
                        };
                        let Some(child) = link_entity_map.get(&joint.child.link) else {
                            warn!("child of joint '{}' not found: {}", joint.name, joint.child.link);
                            continue;
                        };
                        match joint.joint_type {
                            // urdf_rs::JointType::Fixed => {
                            _ => {
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
                    // Add visuals
                    // let link_entity = commands.spawn();
                    // let num = if is_collision {
                    //     l.collision.len()
                    // } else {
                    //     l.visual.len()
                    // };
                    // if num == 0 {
                    //     continue;
                    // }
                    // let mut scene_group = window.add_group();
                    // let mut colors = Vec::new();
                    // for i in 0..num {
                    //     let (geom_element, origin_element) = if is_collision {
                    //         (&l.collision[i].geometry, &l.collision[i].origin)
                    //     } else {
                    //         (&l.visual[i].geometry, &l.visual[i].origin)
                    //     };
                    //     let mut opt_color = None;
                    //     if l.visual.len() > i {
                    //         let rgba = rgba_from_visual(urdf_robot, &l.visual[i]);
                    //         let color = na::Point3::new(rgba[0] as f32, rgba[1] as f32, rgba[2] as f32);
                    //         if color[0] > 0.001 || color[1] > 0.001 || color[2] > 0.001 {
                    //             opt_color = Some(color);
                    //         }
                    //         colors.push(color);
                    //     }
                    //     match add_geometry(
                    //         geom_element,
                    //         &opt_color,
                    //         base_dir,
                    //         &mut scene_group,
                    //         self.is_texture_enabled,
                    //         package_path,
                    //     ) {
                    //         Ok(mut base_group) => {
                    //             // set initial origin offset
                    //             base_group.set_local_transformation(k::urdf::isometry_from(origin_element));
                    //         }
                    //         Err(e) => {
                    //             error!("failed to create for link '{}': {e}", l.name);
                    //         }
                    //     }
                    // }
                    // let joint_name = self
                    //     .link_joint_map
                    //     .get(&l.name)
                    //     .unwrap_or_else(|| panic!("joint for link '{}' not found", l.name));
                    // self.scenes.insert(joint_name.to_owned(), scene_group);
                    // self.original_colors.insert(joint_name.to_owned(), colors);
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

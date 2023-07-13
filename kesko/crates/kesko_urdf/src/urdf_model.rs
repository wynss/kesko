use bevy::{asset::AssetPath, prelude::*, utils::HashMap};
use urdf_rs;

use kesko_core::interaction::groups::GroupDynamic;
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    joint::{fixed::FixedJoint, KeskoAxis},
    mass::Mass,
    rigid_body::RigidBody,
};

use crate::UrdfAsset;

#[derive(Component, Default)]
pub struct IsUrdfSpawned(pub bool);

#[derive(Component)]
pub struct JointName(pub String);

#[derive(Component, Default)]
pub struct UrdfPackageMap(pub HashMap<String, String>);

#[derive(Default, Bundle)]
pub struct UrdfBundle {
    pub urdf_asset: Handle<UrdfAsset>,
    pub urdf_package_map: UrdfPackageMap,
    pub is_urdf_spawned: IsUrdfSpawned,
    pub transform: Transform,
}

pub struct UrdfModel {
    urdf_path: AssetPath<'static>,
    package_map: HashMap<String, String>,
    transform: Transform,
}

impl UrdfModel {
    pub fn new(
        urdf_path: impl Into<AssetPath<'static>>,
        package_map: HashMap<String, String>,
        transform: Transform,
    ) -> Self {
        Self {
            urdf_path: urdf_path.into(),
            package_map: package_map,
            transform,
        }
    }

    pub fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let urdf_asset: Handle<UrdfAsset> = asset_server.load(self.urdf_path.clone());

        println!("urdf_asset: {:?}", urdf_asset);
        // Convert from ROS coordinate frame is Z up, Y left, X forward
        //   to Bevy coordinate frame: Y up, X right, Z backward
        let transform = self.transform
            * Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
            * Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

        commands.spawn((
            UrdfBundle {
                urdf_asset,
                urdf_package_map: UrdfPackageMap(self.package_map.clone()),
                transform,
                ..Default::default()
            },
            RigidBody::Fixed,
        ));
    }
}

pub fn convert_urdf_to_components(
    mut commands: Commands,
    mut urdf_models: Query<(
        Entity,
        &Handle<UrdfAsset>,
        &UrdfPackageMap,
        &mut IsUrdfSpawned,
    )>,
    urdf_assets: Res<Assets<UrdfAsset>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (root_entity, urdf_asset, package_map, mut is_urdf_spawned) in urdf_models.iter_mut() {
        // Substitutes package:// with the path to the package using the package_map
        let asset_path = |urdf_path: &String| -> String {
            let mut processed_path = urdf_path.clone();
            for (package, path) in package_map.0.clone() {
                processed_path =
                    processed_path.replace(format!("package://{package}").as_str(), path.as_str());
            }

            if processed_path.contains("package://") {
                println!(
                    "Warning: package:// still in path: {:?}. Please add the package to the package_map.",
                    processed_path
                );
            }
            processed_path
        };

        if !is_urdf_spawned.0 {
            if let Some(urdf_asset) = urdf_assets.get(urdf_asset) {
                let urdf_robot = urdf_asset.robot.clone();
                // map link name to entity
                let mut link_entity_map: HashMap<String, Entity> = HashMap::default();

                for link in &urdf_robot.links {
                    let part = commands
                        .spawn((
                            kesko_core::bundle::PhysicBodyBundle::from(
                                RigidBody::Dynamic,
                                kesko_core::shape::Shape::Sphere {
                                    radius: 0.01,
                                    subdivisions: 5,
                                },
                                Transform::default(),
                            ),
                            Name::new(link.name.clone()),
                            // RigidBody::Dynamic,
                            // TransformBundle::default(),
                            VisibilityBundle::default(),
                            InteractiveBundle::<GroupDynamic>::default(),
                            // ColliderShape::Sphere {
                            //     radius: 0.2,
                            // },
                            Mass {
                                val: link.inertial.mass.value as f32,
                            },
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
                                commands.entity(visual_entity).insert(PbrBundle {
                                    material: material.clone(),
                                    mesh: asset_server.load(asset_path(filename).as_str()),
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
                        // TODO: figure out how to implement URDF motors properly
                        // urdf_rs::JointType::Revolute => {

                        //     // let STIFFNESS = 3.0;
                        //     // let DAMPING = 0.4;
                        //     // let MAX_MOTOR_FORCE = 70.0;
                        //     let mut physics_joint = RevoluteJoint::attach_to(*parent)
                        //         .with_parent_anchor(urdf_pose_to_transform(&joint.origin))
                        //         .with_axis(urdf_axis_to_kesko_axis(&joint.axis))
                        //         .with_max_motor_force(joint.limit.effort as f32)
                        //         .with_limits(Vec2::new(
                        //             joint.limit.lower as f32,
                        //             joint.limit.upper as f32,
                        //         ))
                        //         // .with_motor_params(STIFFNESS, DAMPING)
                        //         // .with_max_motor_force(MAX_MOTOR_FORCE)
                        //         .with_limits(Vec2::new(-std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4));
                        //     if let Some(dynamics) = joint.dynamics.as_ref() {
                        //         physics_joint = physics_joint.with_motor_params(
                        //             dynamics.friction as f32,
                        //             dynamics.damping as f32,
                        //         );
                        //     }
                        //     commands.entity(*child).insert((physics_joint, JointName(joint.name.clone())));
                        // }
                        // urdf_rs::JointType::Fixed => {
                        _ => {
                            println!("Add fixed joint {} -> {:?}", joint.name, joint.origin);
                            commands.entity(*child).insert(
                                FixedJoint::attach_to(*parent)
                                    .with_parent_anchor(urdf_pose_to_transform(&joint.origin)),
                            );
                        }
                    }
                }
                is_urdf_spawned.0 = true;
            }
        }
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

fn urdf_axis_to_kesko_axis(axis: &urdf_rs::Axis) -> KeskoAxis {
    let x = axis.xyz[0];
    let y = axis.xyz[1];
    let z = axis.xyz[2];
    // check if only one axis is non-zero
    if x != 0.0 && y == 0.0 && z == 0.0 {
        if x > 0.0 {
            KeskoAxis::X
        } else {
            KeskoAxis::NegX
        }
    } else if x == 0.0 && y != 0.0 && z == 0.0 {
        if y > 0.0 {
            KeskoAxis::Y
        } else {
            KeskoAxis::NegY
        }
    } else if x == 0.0 && y == 0.0 && z != 0.0 {
        if z > 0.0 {
            KeskoAxis::Z
        } else {
            KeskoAxis::NegZ
        }
    } else {
        warn!(
            "Invalid axis: {:?}. Only standard basis vectors are supported. Defaulting to X axis",
            axis
        );
        KeskoAxis::X
    }
}

fn urdf_axis_to_kesko_axis_angular(axis: &urdf_rs::Axis) -> KeskoAxis {
    let linear_axis = urdf_axis_to_kesko_axis(axis);
    match linear_axis {
        KeskoAxis::X | KeskoAxis::NegX => KeskoAxis::AngX,
        KeskoAxis::Y | KeskoAxis::NegY => KeskoAxis::AngY,
        KeskoAxis::Z | KeskoAxis::NegZ => KeskoAxis::AngZ,
        _ => linear_axis,
    }
}

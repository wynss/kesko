use std::path::PathBuf;

use bevy::{asset::AssetPath, prelude::*, utils::HashMap};

use kesko_physics::{joint::KeskoAxis, rigid_body::RigidBody};

use crate::SdfAsset;

#[derive(Component, Default)]
pub struct IsSdfSpawned(pub bool);

#[derive(Default, Bundle)]
pub struct SdfBundle {
    pub sdf_asset: Handle<SdfAsset>,
    pub is_sdf_spawned: IsSdfSpawned,
}

pub struct SdfModel {
    sdf_path: PathBuf,
    transform: Transform,
}

impl SdfModel {
    pub fn new(
        sdf_path: impl Into<PathBuf>,
        transform: Transform,
    ) -> Self {
        Self {
            sdf_path: sdf_path.into(),
            transform,
        }
    }

    pub fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let mut sdf_file_path = self.sdf_path.clone();
        sdf_file_path.push("model.sdf");
        println!("load sdf_asset: {:?}", sdf_file_path);
        let sdf_asset: Handle<SdfAsset> = asset_server.load(sdf_file_path);

        
        // Convert from ROS coordinate frame is Z up, Y left, X forward
        //   to Bevy coordinate frame: Y up, X right, Z backward
        let transform = self.transform
            * Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
            * Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

        commands.spawn((
            SdfBundle {
                sdf_asset,
                ..Default::default()
            },
            TransformBundle::from_transform(transform),
            VisibilityBundle::default(),
            RigidBody::Fixed,
        ));
    }
}

pub fn convert_sdf_to_components(
    mut commands: Commands,
    mut sdf_models: Query<(Entity, &Handle<SdfAsset>, &mut IsSdfSpawned)>,
    sdf_assets: Res<Assets<SdfAsset>>,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (root_entity, sdf_asset_handle, mut is_sdf_spawned) in sdf_models.iter_mut() {
        if is_sdf_spawned.0 {
            continue;
        }
        is_sdf_spawned.0 = true;
        let sdf_asset = sdf_assets.get(sdf_asset_handle).unwrap();


        let mut visual_path = sdf_asset.visual_path.clone();
        if visual_path.ends_with(".glb") || visual_path.ends_with(".gltf") {
            visual_path += "#Scene0";
        }
        // Convert from ROS coordinate frame is Z up, Y left, X forward
        //   to Bevy coordinate frame: Y up, X right, Z backward
        let from_ros_frame = 
            Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)) * Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2));
        // let from_ros_frame = Transform::default();   

        commands.entity(root_entity).with_children(|parent| {
            parent.spawn((TransformBundle::from_transform(from_ros_frame), VisibilityBundle::default())).with_children(|parent| {
                parent.spawn(SceneBundle {
                    scene: asset_server.load(visual_path),
                    ..default()
                });
            });
        });
    }
    //     // Substitutes package:// with the path to the package using the package_map
    //     let asset_path = |sdf_path: &String| -> String {
    //         let mut processed_path = sdf_path.clone();
    //         for (package, path) in package_map.0.clone() {
    //             processed_path =
    //                 processed_path.replace(format!("package://{package}").as_str(), path.as_str());
    //         }

    //         if processed_path.contains("package://") {
    //             println!(
    //                 "Warning: package:// still in path: {:?}. Please add the package to the package_map.",
    //                 processed_path
    //             );
    //         }
    //         processed_path
    //     };

    //     if !is_urdf_spawned.0 {
    //         if let Some(urdf_asset) = urdf_assets.get(urdf_asset) {
    //             let urdf_robot = urdf_asset.robot.clone();
    //             // map link name to entity
    //             let mut link_entity_map: HashMap<String, Entity> = HashMap::default();

    //             for link in &urdf_robot.links {
    //                 let part = commands
    //                     .spawn((
    //                         kesko_core::bundle::PhysicBodyBundle::from(
    //                             RigidBody::Dynamic,
    //                             kesko_core::shape::Shape::Sphere {
    //                                 radius: 0.01,
    //                                 subdivisions: 5,
    //                             },
    //                             Transform::default(),
    //                         ),
    //                         Name::new(link.name.clone()),
    //                         // RigidBody::Dynamic,
    //                         // TransformBundle::default(),
    //                         VisibilityBundle::default(),
    //                         InteractiveBundle::<GroupDynamic>::default(),
    //                         // ColliderShape::Sphere {
    //                         //     radius: 0.2,
    //                         // },
    //                         Mass {
    //                             val: link.inertial.mass.value as f32,
    //                         },
    //                     ))
    //                     .id();

    //                 link_entity_map.insert(link.name.clone(), part);

    //                 for visual in link.visual.iter() {
    //                     println!("\n\nvisual: {:?}", visual);
    //                     let transform = urdf_pose_to_transform(&visual.origin);
    //                     let visual_entity = commands
    //                         .spawn((Name::new(
    //                             visual.name.clone().unwrap_or("unnamed_visual".to_string()),
    //                         ),))
    //                         .id();
    //                     commands.entity(part).add_child(visual_entity);

    //                     // Material with the same name in the root of the urdf
    //                     let top_urdf_material = urdf_asset
    //                         .robot
    //                         .materials
    //                         .iter()
    //                         .find(|m| m.name == visual.material.as_ref().unwrap().name);
    //                     let urdf_material = visual.material.as_ref();
    //                     let texture = urdf_material
    //                         .and_then(|m| m.texture.as_ref())
    //                         .or(top_urdf_material.and_then(|m| m.texture.as_ref()));
    //                     let color = urdf_material
    //                         .and_then(|m| m.color.as_ref())
    //                         .or(top_urdf_material.and_then(|m| m.color.as_ref()));

    //                     let material = if let Some(texture) = texture {
    //                         println!("texture: {:?}", texture);
    //                         let texture_path = asset_server.load(texture.filename.as_str());
    //                         materials.add(texture_path.into())
    //                     } else if let Some(color) = color {
    //                         println!("color: {:?}", color);
    //                         materials.add(
    //                             Color::rgba(
    //                                 color.rgba[0] as f32,
    //                                 color.rgba[1] as f32,
    //                                 color.rgba[2] as f32,
    //                                 color.rgba[3] as f32,
    //                             )
    //                             .into(),
    //                         )
    //                     } else {
    //                         materials.add(Color::DARK_GRAY.into())
    //                     };

    //                     print!("processing link: {}", link.name);
    //                     match visual.geometry {
    //                         urdf_rs::Geometry::Sphere { radius } => {
    //                             println!("sphere radius: {}", radius);
    //                             commands.entity(visual_entity).insert(PbrBundle {
    //                                 material: material.clone(),
    //                                 mesh: meshes.add(
    //                                     shape::Icosphere {
    //                                         radius: radius as f32,
    //                                         subdivisions: 5,
    //                                     }
    //                                     .try_into()
    //                                     .unwrap(),
    //                                 ),
    //                                 transform,
    //                                 ..default()
    //                             });
    //                         }
    //                         urdf_rs::Geometry::Box { size } => {
    //                             println!("box size: {:?}", size);
    //                             commands.entity(visual_entity).insert(PbrBundle {
    //                                 material: material.clone(),
    //                                 mesh: meshes.add(
    //                                     shape::Box {
    //                                         min_x: -size.0[0] as f32 / 2.0,
    //                                         min_y: -size.0[1] as f32 / 2.0,
    //                                         min_z: -size.0[2] as f32 / 2.0,
    //                                         max_x: size.0[0] as f32 / 2.0,
    //                                         max_y: size.0[1] as f32 / 2.0,
    //                                         max_z: size.0[2] as f32 / 2.0,
    //                                     }
    //                                     .try_into()
    //                                     .unwrap(),
    //                                 ),
    //                                 transform,
    //                                 ..default()
    //                             });
    //                         }
    //                         urdf_rs::Geometry::Cylinder { radius, length } => {
    //                             println!("cylinder radius: {}, length: {}", radius, length);
    //                         }
    //                         urdf_rs::Geometry::Capsule { radius, length } => {
    //                             println!("capsule radius: {}, length: {}", radius, length);
    //                         }
    //                         urdf_rs::Geometry::Mesh {
    //                             ref filename,
    //                             scale,
    //                         } => {
    //                             commands.entity(visual_entity).insert(PbrBundle {
    //                                 material: material.clone(),
    //                                 mesh: asset_server.load(asset_path(filename).as_str()),
    //                                 transform,
    //                                 ..default()
    //                             });
    //                         }
    //                     };
    //                 }
    //             }

    //             // Find the root link
    //             let links = link_entity_map
    //                 .keys()
    //                 .filter(|name| {
    //                     !urdf_robot
    //                         .joints
    //                         .iter()
    //                         .find(|joint| joint.child.link == **name)
    //                         .is_some()
    //                 })
    //                 .collect::<Vec<_>>();
    //             if links.len() != 1 {
    //                 panic!("expected exactly one root link, found {}", links.len());
    //             }
    //             let Some(root_link) = link_entity_map.get(links[0]) else {
    //                 panic!("Can't find entity for root link '{}'", links[0]);
    //             };
    //             // commands.entity(root_entity).add_child(*root_link);
    //             commands
    //                 .entity(*root_link)
    //                 .insert(FixedJoint::attach_to(root_entity));

    //             for joint in &urdf_robot.joints {
    //                 let Some(parent) = link_entity_map.get(&joint.parent.link) else {
    //                     warn!("parent of joint '{}' not found: {}", joint.name, joint.parent.link);
    //                     continue;
    //                 };
    //                 let Some(child) = link_entity_map.get(&joint.child.link) else {
    //                     warn!("child of joint '{}' not found: {}", joint.name, joint.child.link);
    //                     continue;
    //                 };
    //                 println!("Join {} -> {}", joint.parent.link, joint.child.link);
    //                 match joint.joint_type {
    //                     // TODO: figure out how to implement URDF motors properly
    //                     // urdf_rs::JointType::Revolute => {

    //                     //     // let STIFFNESS = 3.0;
    //                     //     // let DAMPING = 0.4;
    //                     //     // let MAX_MOTOR_FORCE = 70.0;
    //                     //     let mut physics_joint = RevoluteJoint::attach_to(*parent)
    //                     //         .with_parent_anchor(urdf_pose_to_transform(&joint.origin))
    //                     //         .with_axis(urdf_axis_to_kesko_axis(&joint.axis))
    //                     //         .with_max_motor_force(joint.limit.effort as f32)
    //                     //         .with_limits(Vec2::new(
    //                     //             joint.limit.lower as f32,
    //                     //             joint.limit.upper as f32,
    //                     //         ))
    //                     //         // .with_motor_params(STIFFNESS, DAMPING)
    //                     //         // .with_max_motor_force(MAX_MOTOR_FORCE)
    //                     //         .with_limits(Vec2::new(-std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4));
    //                     //     if let Some(dynamics) = joint.dynamics.as_ref() {
    //                     //         physics_joint = physics_joint.with_motor_params(
    //                     //             dynamics.friction as f32,
    //                     //             dynamics.damping as f32,
    //                     //         );
    //                     //     }
    //                     //     commands.entity(*child).insert((physics_joint, JointName(joint.name.clone())));
    //                     // }
    //                     // urdf_rs::JointType::Fixed => {
    //                     _ => {
    //                         println!("Add fixed joint {} -> {:?}", joint.name, joint.origin);
    //                         commands.entity(*child).insert(
    //                             FixedJoint::attach_to(*parent)
    //                                 .with_parent_anchor(urdf_pose_to_transform(&joint.origin)),
    //                         );
    //                     }
    //                 }
    //             }
    //             is_urdf_spawned.0 = true;
    //         }
    //     }
    // }
}


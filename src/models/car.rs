use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use nora_physics::{
    rigid_body::RigidBody,
    joint::{
        Joint,
        fixed::FixedJoint,
        revolute::RevoluteJoint
    }
};
use nora_core::{
    bundle::PhysicBodyBundle,
    shape::Shape,
    interaction::groups::GroupDynamic,
    transform::get_world_transform
};
use nora_object_interaction::InteractiveBundle;


pub fn spawn_car(
    commands: &mut Commands,
    material_body: Handle<StandardMaterial>,
    material_wheel: Handle<StandardMaterial>,
    origin: Transform,
    meshes: &mut Assets<Mesh>
) {
    let frame_width = 0.5;
    let frame_height = 0.1;
    let frame_length = 1.0;

    let half_frame_length = frame_length / 2.0;
    let half_frame_width = frame_width / 2.0;
    let half_frame_height = frame_height / 2.0;

    let wheel_radius = 0.18;
    let wheel_width = 0.08;
    let wheel_base = frame_width + 0.2;
    let wbh = wheel_base / 2.0;

    let wall_thickness = 0.05;
    let wall_height = 0.2;
    let half_wall_height = wall_height / 2.0;
    let half_wall_thick = wall_thickness / 2.0;

    // Frame
    let frame = commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: frame_width, y_length: frame_height, z_length: frame_length},
        material_body.clone(),
        origin,
        meshes
    ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .id();

    // front wall
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, half_frame_height, half_frame_length - half_wall_thick));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: frame_width, y_length: wall_height, z_length: wall_thickness},
        material_body.clone(),
        world_transform,
        meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

    // back wall
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, frame_height / 2.0, -(half_frame_length - half_wall_thick)));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: frame_width, y_length: wall_height, z_length: wall_thickness},
        material_body.clone(),
        world_transform,
        meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

    // left wall
    let parent_anchor = Transform::from_translation(Vec3::new(half_frame_width - half_wall_thick, frame_height / 2.0, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: wall_thickness, y_length: wall_height, z_length: frame_length - 2.0 * wall_thickness},
        material_body.clone(),
        world_transform,
        meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

    // right wall
    let parent_anchor = Transform::from_translation(Vec3::new(-half_frame_width + half_wall_thick, frame_height / 2.0, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: wall_thickness, y_length: wall_height, z_length: frame_length - 2.0 * wall_thickness},
        material_body.clone(),
        world_transform,
        meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

    // left front wheel
    let parent_anchor = Transform::from_translation(Vec3::new(wbh, 0.0, half_frame_length));
    let child_anchor = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: wheel_radius, length: wheel_width},
        material_wheel.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }));

    // right front wheel
    let parent_anchor = Transform::from_translation(Vec3::new(-wbh, 0.0, half_frame_length));
    let child_anchor = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: wheel_radius, length: wheel_width},
        material_wheel.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }));

    // left back wheel
    let parent_anchor = Transform::from_translation(Vec3::new(wbh, 0.0, -half_frame_length));
    let child_anchor = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: wheel_radius, length: wheel_width},
        material_wheel.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }));

    // right back wheel
    let parent_anchor = Transform::from_translation(Vec3::new(-wbh, 0.0, -half_frame_length));
    let child_anchor = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: wheel_radius, length: wheel_width},
        material_wheel.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }));
}

// /// Controller for the car model
// /// Should hold the settings for sensitivity etc
// #[derive(Component)]
// struct CarController;

// fn send_car_control_events(
//     keys: Res<Input<KeyCode>>,
//     car_event_writer: EventWriter<CarEvent>,
//     query: Query<Entity, With<CarController>>
// ) {

//     if keys.pressed(KeyCode::W) {

//     } else if keys.pressed(KeyCode::S) {
        
//     } else if keys.pressed(KeyCode::A) {
        
//     } else if keys.pressed(KeyCode::D) {
        
//     }
// }

// fn handle_car_control_system(
//     mut event_reader: EventReader<CarEvent>,
//     query: Query<(Entity, &Multibody), With<CarController>>
// ) {

//     let (e, multi_body) = query.get_single().unwrap();

//     for event in event_reader.iter() {
//         match event {
//             Forward => {
                
//                 let wheel_entity = car_body.joints["rear_left_wheel"];
//                 writer.send(JointEvent::SetVelocityRevolute {
//                     entity: wheel_entity, 
//                     vel: 3.0
//                 })
//             }
//         }
//     }
// }

// enum CarEvent {
//     Forward,
//     Backward,
//     Left,
//     Right
// }

// #[derive(Component)]
// struct CarId(u32);
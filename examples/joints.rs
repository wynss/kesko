use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
use bevy::prelude::*;
use nora_core::{
    diagnostic::event::DebugEventPlugin,
    plugins::CorePlugins,
    interaction::groups::GroupDynamic,
    shape::Shape,
    bundle::MeshPhysicBodyBundle,
};
use nora_physics::{
    rigid_body::{RigidBody, CanSleep},
    gravity::GravityScale, 
    joint::{
        Joint,
        revolute::RevoluteJoint, 
        JointMotorEvent, 
        MotorAction
    }
};
use nora_object_interaction::InteractiveBundle;

const STIFFNESS: f32 = 1.0;
const DAMPING: f32 = 0.1;


#[derive(Component)]
struct TestComp;

#[derive(Component)]
struct TurnComp;

#[derive(Component)]
struct LinkXComp;

#[derive(Component)]
struct LinkZComp;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CorePlugins)
    .add_plugin(DebugEventPlugin)
    .add_startup_system(setup_scene)
    .add_system(set_velocity)
    .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    let root = commands.spawn().insert_bundle(PbrBundle {
        material: materials.add(Color::ALICE_BLUE.into()),
        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(CanSleep(false))
    .id();

    let link = commands.spawn_bundle( MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder {radius: 0.1, length: 0.1, resolution: 5},
        materials.add(Color::ALICE_BLUE.into()),
        Transform::default(),
        &mut meshes
    ))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(TurnComp)
    .insert(CanSleep(false))
    .insert(Joint::new(
        root,
        RevoluteJoint {
            parent_anchor: Transform::from_translation(Vec3::new(-1.0, 0.0, 0.0)).with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
            child_anchor: Transform::default(),
            axis: Vec3::X,
            limits: Some(Vec2::new(-FRAC_PI_4, FRAC_PI_4)),
            ..default()
        }
    )).id();

    commands.spawn_bundle( MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder {radius: 0.3, length: 0.1, resolution: 5},
        materials.add(Color::ALICE_BLUE.into()),
        Transform::from_translation(Vec3::new(-1.4, 0.0, 0.0)),
        &mut meshes
    ))
    .insert(GravityScale::new(0.0))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(TestComp)
    .insert(CanSleep(false))
    .insert(Joint::new(
        link,
        RevoluteJoint {
            parent_anchor: Transform::from_translation(Vec3::new(0.0, 0.11, 0.0)),
            child_anchor: Transform::default(),
            axis: Vec3::Y,
            ..default()
        }
    ));

    
    let base = commands.spawn_bundle( MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Sphere { radius: 0.1, subdivisions: 5 },
        materials.add(Color::ALICE_BLUE.into()),
        Transform::default(),
        &mut meshes
    ))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(CanSleep(false))
    .insert(LinkZComp)
    .insert(Joint::new(
        root,
        RevoluteJoint {
            parent_anchor: Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
            child_anchor: Transform::default(),
            axis: Vec3::Z,
            stiffness: 100.0,
            damping: 100.0,
            ..default()
        }
    )).id();

    commands.spawn_bundle( MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule {radius: 0.1, length: 0.5},
        materials.add(Color::ALICE_BLUE.into()),
        Transform::default(),
        &mut meshes
    ))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(CanSleep(false))
    .insert(LinkXComp)
    .insert(Joint::new(
        base,
        RevoluteJoint {
            parent_anchor: Transform::from_xyz(0.0, 0.0, 0.0),
            child_anchor: Transform::from_translation(Vec3::new(0.0, -0.47, 0.0)),
            axis: Vec3::X,
            stiffness: 100.0,
            damping: 100.0,
            ..default()
        }
    ));


    // poker
    commands.spawn_bundle( MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Sphere { radius: 0.1, subdivisions: 5 },
        materials.add(Color::PINK.into()),
        Transform::from_xyz(2.0, 0.0, 0.0),
        &mut meshes
    ))
    .insert(GravityScale::new(0.0))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(20.0, 40.0, -40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}


fn set_velocity(
    keys: Res<Input<KeyCode>>,
    mut joint_event: EventWriter<JointMotorEvent>,
    spin_query: Query<Entity, With<TestComp>>,
    turn_query: Query<Entity, With<TurnComp>>,
    link_z_query: Query<Entity, With<LinkZComp>>,
    link_x_query: Query<Entity, With<LinkXComp>>
 ) {

    let spin_entity = spin_query.get_single().unwrap();
    let turn_entity = turn_query.get_single().unwrap();
    let link_x_entity = link_x_query.get_single().unwrap();
    let link_z_entity = link_z_query.get_single().unwrap();

    if keys.just_pressed(KeyCode::W) {
        joint_event.send( JointMotorEvent {
            entity: spin_entity,
            action: MotorAction::VelocityRevolute { velocity: 3.0, factor: 30.0 }
        });
        joint_event.send( JointMotorEvent {
            entity: link_z_entity,
            action: MotorAction::PositionRevolute { position: -FRAC_PI_4, damping: DAMPING, stiffness: STIFFNESS }
        });
    } else if keys.just_pressed(KeyCode::S) {
        joint_event.send( JointMotorEvent {
            entity: spin_entity,
            action: MotorAction::VelocityRevolute { velocity: -3.0, factor: 30.0 }
        });
        joint_event.send( JointMotorEvent {
            entity: link_z_entity,
            action: MotorAction::PositionRevolute { position: FRAC_PI_4, damping: DAMPING, stiffness: STIFFNESS }
        });
    } else if keys.just_released(KeyCode::S) || keys.just_released(KeyCode::W) {
        joint_event.send( JointMotorEvent {
            entity: spin_entity,
            action: MotorAction::VelocityRevolute { velocity: 0.0, factor: 30.0 }
        });
        joint_event.send( JointMotorEvent {
            entity: link_z_entity,
            action: MotorAction::PositionRevolute { position: 0.0, damping: DAMPING, stiffness: STIFFNESS }
        });
    }

    if keys.just_pressed(KeyCode::A) {
        joint_event.send( JointMotorEvent {
            entity: turn_entity,
            action: MotorAction::PositionRevolute { position: -FRAC_PI_4, damping: DAMPING, stiffness: STIFFNESS }
        });
        joint_event.send( JointMotorEvent {
            entity: link_x_entity,
            action: MotorAction::PositionRevolute { position: -FRAC_PI_4, damping: DAMPING, stiffness: STIFFNESS }
        });
    } else if keys.just_pressed(KeyCode::D) {
        joint_event.send( JointMotorEvent {
            entity: turn_entity,
            action: MotorAction::PositionRevolute { position: FRAC_PI_4, damping: DAMPING, stiffness: STIFFNESS }
        });
        joint_event.send( JointMotorEvent {
            entity: link_x_entity,
            action: MotorAction::PositionRevolute { position: FRAC_PI_4, damping: DAMPING, stiffness: STIFFNESS }
        });
    } else if keys.just_released(KeyCode::D) || keys.just_released(KeyCode::A) {
        joint_event.send( JointMotorEvent {
            entity: turn_entity,
            action: MotorAction::PositionRevolute { position: 0.0, damping: DAMPING, stiffness: STIFFNESS }
        });
        joint_event.send( JointMotorEvent {
            entity: link_x_entity,
            action: MotorAction::PositionRevolute { position: 0.0, damping: DAMPING, stiffness: STIFFNESS }
        });
    }
}

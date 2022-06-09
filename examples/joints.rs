use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use kesko_physics::{
    rigid_body::{
        RigidBody,
        RigidBodyName
    },
    joint::{
        Joint,
        revolute::RevoluteJoint,
        fixed::FixedJoint, spherical::SphericalJoint
    }, mass::Mass
};
use kesko_object_interaction::InteractiveBundle;
use kesko_core::{
    shape::Shape,
    bundle::MeshPhysicBodyBundle,
    transform::get_world_transform,
    interaction::groups::GroupDynamic
};
use kesko_models::arena::spawn_arena;
use kesko_plugins::CorePlugins;


fn main() {
    App::new()
    .add_plugins(CorePlugins)
    .add_startup_system(setup_scene)
    .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    spawn_arena(
        &mut commands, 
        materials.add(Color::CRIMSON.into()),
        &mut meshes,
        10.0, 10.0, 0.0
    );

    let material = materials.add(Color::BLUE.into());
    let origin = Transform::default();

    let bench = build_test_bench(origin, &mut commands, &material, &mut meshes);

    let parent_anchor = Transform::from_translation(Vec3::new(-4.0, 1.2, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.1, y_length: 1.0, z_length: 0.1 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
    .insert(Joint::new(bench, SphericalJoint {
        parent_anchor,
        child_anchor,
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("spherical".to_owned()));
    
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 1.2, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.1, y_length: 1.0, z_length: 0.1 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
    .insert(Joint::new(bench, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::Y,
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("world_aligned_y".to_owned()));
    
    let parent_anchor = Transform::from_translation(Vec3::new(2.0, 1.2, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.1, y_length: 1.0, z_length: 0.1 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
    .insert(Joint::new(bench, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::X,
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("world_aligned_x".to_owned()));

    let parent_anchor = Transform::from_translation(Vec3::new(-2.0, 1.2, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.1, y_length: 1.0, z_length: 0.1 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
    .insert(Joint::new(bench, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::Z,
        damping: 0.1,
        stiffness: 1.0,
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("world_aligned_z".to_owned()));
    
    let parent_anchor = Transform::from_translation(Vec3::new(1.0, 1.2, 2.0)).with_rotation(Quat::from_rotation_x(FRAC_PI_2));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.1, y_length: 1.0, z_length: 0.1 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
    .insert(Joint::new(bench, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::X,
        damping: 0.1,
        stiffness: 1.0,
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("x_90_x_parent_rot".to_owned()));
    
    let parent_anchor = Transform::from_translation(Vec3::new(-1.0, 1.2, 2.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)).with_rotation(Quat::from_rotation_x(FRAC_PI_2));
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.1, y_length: 1.0, z_length: 0.1 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
    .insert(Joint::new(bench, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::X,
        damping: 0.1,
        stiffness: 1.0,
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("x_90_x_child_rot".to_owned()));


    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 1.2, -2.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    let stick_1 = commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.1, y_length: 1.0, z_length: 0.1 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
    .insert(Joint::new(bench, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::Z,
        damping: 0.1,
        stiffness: 1.0,
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("stick_1_z".to_owned()))
    .id();

    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.6, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.6, 0.0));
    let stick_2 = commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.1, y_length: 1.0, z_length: 0.1 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
    .insert(Joint::new(stick_1, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::X,
        damping: 0.1,
        stiffness: 1.0,
        limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("stick_2_x".to_owned()))
    .id();

    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.6, 0.0)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.6, 0.0));
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.1, y_length: 1.0, z_length: 0.1 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
    .insert(Joint::new(stick_2, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::Y,
        damping: 0.1,
        stiffness: 1.0,
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("stick_3_y".to_owned()));


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


fn build_test_bench(
    origin: Transform,
    commands: &mut Commands,
    material: &Handle<StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> Entity {

    //base 
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.75, 0.0));
    let child_anchor = Transform::default();
    let base = commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 4.0, y_length: 1.0, z_length: 4.0 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
    ))
    .insert(Mass { val: 100000.0 })
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("base".to_owned()))
    .id();

    // base pole
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 1.0, 0.0));
    let child_anchor = Transform::default();
    let base_pole = commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 0.2, y_length: 1.0, z_length: 0.2 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
    ))
    .insert(Joint::new(base, FixedJoint {
        parent_anchor,
        child_anchor
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("base".to_owned()))
    .id();
    
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.60, 0.0));
    let child_anchor = Transform::default();
    let bench = commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box { x_length: 6.0, y_length: 0.2, z_length: 0.2 },
        material.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
    ))
    .insert(Joint::new(base_pole, FixedJoint {
        parent_anchor,
        child_anchor
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("bench".to_owned()))
    .id();

    bench
}
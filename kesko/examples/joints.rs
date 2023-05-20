use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use kesko_core::{
    bundle::MeshPhysicBodyBundle, interaction::groups::GroupDynamic, shape::Shape,
    transform::world_transform_from_joint_anchors,
};
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    joint::{fixed::FixedJoint, revolute::RevoluteJoint, spherical::SphericalJoint, KeskoAxis},
    mass::Mass,
    rigid_body::RigidBody,
};
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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    kesko_models::plane::spawn(
        &mut commands,
        materials.add(Color::CRIMSON.into()),
        &mut meshes,
    );

    let material = materials.add(Color::BLUE.into());
    let origin = Transform::default();

    let (bench, world_transform) = build_test_bench(origin, &mut commands, &material, &mut meshes);

    let parent_anchor = Transform::from_translation(Vec3::new(-4.0, 1.2, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn((
        MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {
                x_length: 0.1,
                y_length: 1.0,
                z_length: 0.1,
            },
            material.clone(),
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            &mut meshes,
        ),
        SphericalJoint::attach_to(bench)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor),
        InteractiveBundle::<GroupDynamic>::default(),
        Name::new("spherical"),
    ));

    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 1.2, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn((
        MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {
                x_length: 0.1,
                y_length: 1.0,
                z_length: 0.1,
            },
            material.clone(),
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            &mut meshes,
        ),
        RevoluteJoint::attach_to(bench)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::Y)
            .with_motor_params(1.0, 0.1),
        InteractiveBundle::<GroupDynamic>::default(),
        Name::new("world_aligned_y"),
    ));

    let parent_anchor = Transform::from_translation(Vec3::new(2.0, 1.2, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn((
        MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {
                x_length: 0.1,
                y_length: 1.0,
                z_length: 0.1,
            },
            material.clone(),
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            &mut meshes,
        ),
        RevoluteJoint::attach_to(bench)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::X)
            .with_motor_params(1.0, 0.1),
        InteractiveBundle::<GroupDynamic>::default(),
        Name::new("world_aligned_x"),
    ));

    let parent_anchor = Transform::from_translation(Vec3::new(-2.0, 1.2, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn((
        MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {
                x_length: 0.1,
                y_length: 1.0,
                z_length: 0.1,
            },
            material.clone(),
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            &mut meshes,
        ),
        RevoluteJoint::attach_to(bench)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::Z)
            .with_motor_params(1.0, 0.1),
        InteractiveBundle::<GroupDynamic>::default(),
        Name::new("world_aligned_z"),
    ));

    let parent_anchor = Transform::from_translation(Vec3::new(1.0, 1.2, 2.0))
        .with_rotation(Quat::from_rotation_x(FRAC_PI_2));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    commands.spawn((
        MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {
                x_length: 0.1,
                y_length: 1.0,
                z_length: 0.1,
            },
            material.clone(),
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            &mut meshes,
        ),
        RevoluteJoint::attach_to(bench)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::X)
            .with_motor_params(1.0, 0.1),
        InteractiveBundle::<GroupDynamic>::default(),
        Name::new("x_90_x_parent_rot"),
    ));

    let parent_anchor = Transform::from_translation(Vec3::new(-1.0, 1.2, 2.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0))
        .with_rotation(Quat::from_rotation_x(FRAC_PI_2));
    commands.spawn((
        MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {
                x_length: 0.1,
                y_length: 1.0,
                z_length: 0.1,
            },
            material.clone(),
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            &mut meshes,
        ),
        RevoluteJoint::attach_to(bench)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::X)
            .with_motor_params(1.0, 0.1),
        InteractiveBundle::<GroupDynamic>::default(),
        Name::new("x_90_x_child_rot"),
    ));

    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 1.2, -2.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.5, 0.0));
    let world_transform =
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
    let stick_1 = commands
        .spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: 0.1,
                    y_length: 1.0,
                    z_length: 0.1,
                },
                material.clone(),
                world_transform,
                &mut meshes,
            ),
            RevoluteJoint::attach_to(bench)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(1.0, 0.1),
            InteractiveBundle::<GroupDynamic>::default(),
            Name::new("stick_1_z"),
        ))
        .id();

    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.6, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.6, 0.0));
    let world_transform =
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
    let stick_2 = commands
        .spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: 0.1,
                    y_length: 1.0,
                    z_length: 0.1,
                },
                material.clone(),
                world_transform,
                &mut meshes,
            ),
            RevoluteJoint::attach_to(stick_1)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(1.0, 0.1)
                .with_limits(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            InteractiveBundle::<GroupDynamic>::default(),
            Name::new("stick_2_x"),
        ))
        .id();

    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.6, 0.0))
        .with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.6, 0.0));
    let world_transform =
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
    let stick_3 = commands
        .spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: 0.1,
                    y_length: 1.0,
                    z_length: 0.1,
                },
                material.clone(),
                world_transform,
                &mut meshes,
            ),
            RevoluteJoint::attach_to(stick_2)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Y)
                .with_motor_params(1.0, 0.1),
            InteractiveBundle::<GroupDynamic>::default(),
            Name::new("stick_3_y"),
        ))
        .id();

    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.6, 0.0))
        .with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -0.6, 0.0));
    let world_transform =
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
    commands.spawn((
        MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {
                x_length: 0.1,
                y_length: 1.0,
                z_length: 0.1,
            },
            material.clone(),
            world_transform,
            &mut meshes,
        ),
        RevoluteJoint::attach_to(stick_3)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::Y)
            .with_motor_params(1.0, 0.1),
        InteractiveBundle::<GroupDynamic>::default(),
        Name::new("stick_4_y"),
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
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
) -> (Entity, Transform) {
    //base
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.75, 0.0));
    let child_anchor = Transform::default();
    let world_transform =
        world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor);
    let base = commands
        .spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: 4.0,
                    y_length: 1.0,
                    z_length: 4.0,
                },
                material.clone(),
                world_transform,
                meshes,
            ),
            Mass { val: 10000000.0 },
            InteractiveBundle::<GroupDynamic>::default(),
            Name::new("base"),
        ))
        .id();

    // base pole
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 1.0, 0.0));
    let child_anchor = Transform::default();
    let world_transform =
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
    let base_pole = commands
        .spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: 0.2,
                    y_length: 1.0,
                    z_length: 0.2,
                },
                material.clone(),
                world_transform,
                meshes,
            ),
            FixedJoint::attach_to(base)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor),
            InteractiveBundle::<GroupDynamic>::default(),
            Name::new("base"),
        ))
        .id();

    let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.60, 0.0));
    let child_anchor = Transform::default();
    let world_transform =
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
    let bench = commands
        .spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: 6.0,
                    y_length: 0.2,
                    z_length: 0.2,
                },
                material.clone(),
                world_transform,
                meshes,
            ),
            FixedJoint::attach_to(base_pole)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor),
            InteractiveBundle::<GroupDynamic>::default(),
            Name::new("bench"),
        ))
        .id();

    (bench, world_transform)
}

use bevy::{prelude::*, render::{texture::ImageSettings, render_resource::{SamplerDescriptor, AddressMode}}};

use kesko_core::{
    interaction::groups::GroupDynamic, 
};

use kesko_plugins::CorePlugins;
use kesko_diagnostic::DiagnosticsPlugins;

use kesko_models::{
    self,
    car::CarPlugin,
    wheely::WheelyPlugin
};
use kesko_physics::{
    rigid_body::RigidBody,
    force::Force,
    gravity::GravityScale,
    collider::ColliderShape,
    event::collision::GenerateCollisionEvents
};
use kesko_object_interaction::InteractiveBundle;


fn main() {
    App::new()
    .add_plugins(CorePlugins)
    .add_plugins(DiagnosticsPlugins)
    .add_plugin(CarPlugin)
    .add_plugin(WheelyPlugin)
    .add_startup_system(test_scene)
    .insert_resource(ImageSettings {
        default_sampler: SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            ..Default::default()
        },
    })
    .run();
}


fn test_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    kesko_models::plane::spawn(
        &mut commands,
        materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        }), 
        &mut meshes,
    );

    kesko_models::car::Car::spawn(
        &mut commands,
        materials.add(Color::SEA_GREEN.into()), 
        materials.add(Color::DARK_GRAY.into()), 
        Transform::from_xyz(0.0, 1.0, -2.0),
        &mut meshes,
    );

    kesko_models::spider::spawn(
        &mut commands,
        materials.add(Color::CRIMSON.into()), 
        Transform::from_xyz(2.0, 1.0, 0.0),
        &mut meshes,
    );

    kesko_models::wheely::Wheely::spawn(
        &mut commands,
        materials.add(Color::FUCHSIA.into()), 
        materials.add(Color::DARK_GRAY.into()), 
        Transform::from_xyz(-2.0, 1.0, 0.0),
        &mut meshes,
    );

    // kesko_models::humanoid::Humanoid::spawn(
    //     &mut commands,
    //     materials.add(StandardMaterial { 
    //         base_color: Color::ORANGE,
    //         perceptual_roughness: 1.0,
    //         ..default()
    //     }), 
    //     Transform::from_xyz(2.0, 2.0, 2.0),
    //     &mut meshes
    // );

    // // spawn sphere that will generate collision events
    // commands.spawn_bundle(PbrBundle {
    //     material: materials.add(Color::PURPLE.into()),
    //     mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.2, subdivisions: 5})),
    //     transform: Transform::from_translation(Vec3::new(0.0, 1.0, 2.0)),
    //     ..default()
    // })
    // .insert(RigidBody::Dynamic)
    // .insert(ColliderShape::Sphere { radius: 0.2 })
    // .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    // .insert(Force::default())
    // .insert(GravityScale::default())
    // .insert(GenerateCollisionEvents);

    // Light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}

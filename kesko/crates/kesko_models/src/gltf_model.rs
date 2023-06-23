use bevy::{prelude::*, asset::AssetPath};

use kesko_core::interaction::groups::GroupDynamic;
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    collider::{ColliderPhysicalProperties, ColliderShape},
    force::Force,
    gravity::GravityScale,
    rigid_body::RigidBody,
};

pub struct GltfModel;
impl GltfModel {
    pub fn spawn(commands: &mut Commands, asset_path: impl Into<AssetPath<'static>>, transform: Transform, asset_server: & Res<AssetServer>) {
        let my_gltf = asset_server.load(
            asset_path,
            // "/home/azazdeaz/repos/temp/bevy/assets/models/FlightHelmet/FlightHelmet.gltf#Scene0",
        );

        commands.spawn((
            SceneBundle {
                scene: my_gltf,
                transform,
                ..Default::default()
            },
            RigidBody::Dynamic,
            ColliderShape::Sphere { radius: 0.2 },
            InteractiveBundle::<GroupDynamic>::default(),
            Force::default(),
            ColliderPhysicalProperties {
                restitution: 0.7,
                ..default()
            },
            GravityScale::default(),
        ));
    }
}

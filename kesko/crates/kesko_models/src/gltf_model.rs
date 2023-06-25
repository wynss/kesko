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
        let gltf_asset = asset_server.load(
            asset_path,
        );
        // TODO: add option to select collider submeshes

        commands.spawn((
            SceneBundle {
                scene: gltf_asset,
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

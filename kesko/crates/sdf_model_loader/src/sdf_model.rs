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

        commands.spawn((
            SdfBundle {
                sdf_asset,
                ..Default::default()
            },
            TransformBundle::from_transform(self.transform),
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
        if let Some(sdf_asset) = sdf_assets.get(sdf_asset_handle) {
            is_sdf_spawned.0 = true;

            let mut visual_path = sdf_asset.visual_path.clone();
            if visual_path.ends_with(".glb") || visual_path.ends_with(".gltf") {
                visual_path += "#Scene0";
            }
            // Convert from ROS coordinate frame is Z up, Y left, X forward
            //   to Bevy coordinate frame: Y up, X right, Z backward
            let from_ros_frame = Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
            * Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
                // Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)) * Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2));
            let from_ros_frame = Transform::default();   

            commands.entity(root_entity).with_children(|parent| {
                parent.spawn((TransformBundle::from_transform(from_ros_frame), VisibilityBundle::default())).with_children(|parent| {
                    parent.spawn(SceneBundle {
                        scene: asset_server.load(visual_path),
                        ..default()
                    });
                });
            });
        }
    }
}


use bevy::prelude::*;


#[derive(Component, Default)]
pub(crate) struct OriginalMaterial(pub(crate) Option<Handle<StandardMaterial>>);

pub(crate) struct InteractionMaterials {
    pub(crate) hovered: Handle<StandardMaterial>,
    pub(crate) pressed: Handle<StandardMaterial>,
}

impl FromWorld for InteractionMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        Self {
            hovered: materials.add(Color::GOLD.into()),
            pressed: materials.add(Color::INDIGO.into()),
        }
    }
}

pub(crate) fn set_initial_interaction_material(
    mut query: Query<(&mut OriginalMaterial, &Handle<StandardMaterial>)>,
) {
    for (mut original_material, material) in query.iter_mut() {
        if original_material.0.is_none() {
            original_material.0 = Some(material.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::asset::AssetPlugin;
    use bevy::core::CorePlugin;
    use bevy::core_pipeline::CorePipelinePlugin;
    use bevy::pbr::PbrPlugin;
    use bevy::render::RenderPlugin;
    use bevy::window::WindowPlugin;
    use crate::{OriginalMaterial, set_initial_interaction_material};

    #[test]
    fn test_set_initial_material() {

        let mut app  = App::new();
        app.add_plugin(CorePlugin)
            .add_plugin(AssetPlugin)
            .add_plugin(WindowPlugin::default())
            .add_plugin(RenderPlugin)
            .add_plugin(CorePipelinePlugin)
            .add_plugin(PbrPlugin);

        let world = &mut app.world;
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(Color::GOLD.into());
        world.spawn().insert(OriginalMaterial::default()).insert(material);

        let mut stage = SystemStage::parallel();
        stage.add_system(set_initial_interaction_material);

        stage.run(world);

        // only 1 entity
        assert_eq!(world.query::<&OriginalMaterial>().iter(world).len(), 1);

        world.query::<(&OriginalMaterial, &Handle<StandardMaterial>)>()
            .for_each(world, |(original_material, material)| {

                // assert we have a material set
                assert!(original_material.0.is_some());

                // assert that is the same as the material assigned to the entity
                if let Some(original_material) = &original_material.0 {
                    assert_eq!(original_material, material);
                }
        });
    }
}

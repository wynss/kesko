use bevy::prelude::*;

#[derive(Component, Default)]
pub(crate) struct OriginalMaterial(pub(crate) Option<Handle<StandardMaterial>>);

#[derive(Resource)]
pub(crate) struct InteractionMaterials {
    pub(crate) hovered: Handle<StandardMaterial>,
    pub(crate) dragged: Handle<StandardMaterial>,
    pub(crate) selected: Handle<StandardMaterial>,
}

impl FromWorld for InteractionMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        Self {
            hovered: materials.add(Color::hex("81D4FA").unwrap().into()),
            dragged: materials.add(Color::hex("03A9F4").unwrap().into()),
            selected: materials.add(Color::GOLD.into()),
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
    use crate::{set_initial_interaction_material, OriginalMaterial};
    use bevy::asset::AssetPlugin;
    use bevy::core::TypeRegistrationPlugin;
    use bevy::prelude::*;

    #[test]
    fn test_set_initial_material() {
        let mut app = App::new();
        app.add_plugin(TaskPoolPlugin::default())
            .add_plugin(TypeRegistrationPlugin::default())
            .add_plugin(AssetPlugin::default())
            .add_asset::<StandardMaterial>();

        let mut materials = app.world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(Color::GOLD.into());
        app.world.spawn((OriginalMaterial::default(), material));

        app.add_system(set_initial_interaction_material);
        app.update();

        // only 1 entity
        assert_eq!(
            app.world
                .query::<&OriginalMaterial>()
                .iter(&app.world)
                .len(),
            1
        );

        app.world
            .query::<(&OriginalMaterial, &Handle<StandardMaterial>)>()
            .for_each(&app.world, |(original_material, material)| {
                // assert we have a material set
                assert!(original_material.0.is_some());

                // assert that is the same as the material assigned to the entity
                if let Some(original_material) = &original_material.0 {
                    assert_eq!(original_material, material);
                }
            });
    }
}

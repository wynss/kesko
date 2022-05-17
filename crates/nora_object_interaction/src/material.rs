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

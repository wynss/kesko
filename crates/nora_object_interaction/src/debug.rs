use bevy::prelude::*;
use crate::{InteractionState, InteractionMaterials, OriginalMaterial};


pub(crate) fn update_interaction_material(
    interaction_materials: Res<InteractionMaterials>,
    mut interaction_query: Query<(&InteractionState, &OriginalMaterial, &mut Handle<StandardMaterial>), Changed<InteractionState>>,
) {
    for (interaction, original_material, mut current_material) in interaction_query.iter_mut() {
        match interaction {
            InteractionState::Pressed => *current_material = interaction_materials.pressed.clone(),
            InteractionState::Hovered => *current_material = interaction_materials.hovered.clone(),
            InteractionState::None => {
                if let Some(material) = &original_material.0 {
                   *current_material = material.clone();
                }
            }
        }
    }
}

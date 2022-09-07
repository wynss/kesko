use bevy::prelude::*;
use crate::{
    event::InteractionEvent,
    material::{InteractionMaterials, OriginalMaterial}, interaction::{Drag, Select, Hover}
};


#[allow(clippy::type_complexity)]
pub(crate) fn update_interaction_material<T: Component + Default>(
    event_reader: EventReader<InteractionEvent>,
    interaction_materials: Res<InteractionMaterials>,
    mut material_query: Query<(&mut Handle<StandardMaterial>, &OriginalMaterial), With<OriginalMaterial>>,
    interactive_entities: Query<(Entity, &Drag<T>, &Select<T>, &Hover<T>), With<OriginalMaterial>>
) {

    if !event_reader.is_empty() {

        for (entity, drag, select, hover) in interactive_entities.iter() {
            if let Ok((mut current_material, original_material)) = material_query.get_mut(entity) {
                if drag.dragged {
                    *current_material = interaction_materials.dragged.clone();
                } else if hover.hovered {
                    *current_material = interaction_materials.hovered.clone();
                } else if select.selected {
                    *current_material = interaction_materials.selected.clone();
                } else if let Some(material) = &original_material.0 {
                    *current_material = material.clone();
                }
            }
        }
    }
}

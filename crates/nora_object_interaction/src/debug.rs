use bevy::prelude::*;
use crate::{
    event::{
        InteractionEvent, DragEvent, HoverEvent
    },
    material::{InteractionMaterials, OriginalMaterial}
};


pub(crate) fn update_interaction_material(
    mut event_reader: EventReader<InteractionEvent>,
    interaction_materials: Res<InteractionMaterials>,
    mut material_query: Query<(&mut Handle<StandardMaterial>, &OriginalMaterial), With<OriginalMaterial>>
) {

    for event in event_reader.iter() {
        match event {
            InteractionEvent::Drag(drag_event) => if let DragEvent::Started(e)=  drag_event {
                let (mut current_material, _) = material_query.get_mut(*e).unwrap();
                *current_material = interaction_materials.pressed.clone();
            },
            InteractionEvent::Hover(hover_event) => if let HoverEvent::Started(e) = hover_event {
                let (mut current_material, _) = material_query.get_mut(*e).unwrap();
                *current_material = interaction_materials.hovered.clone();
            },
            InteractionEvent::NoInteraction(e) => {
                let (mut current_material, original_material) = material_query.get_mut(*e).unwrap();
                if let Some(material) = &original_material.0 {
                    *current_material = material.clone();
                }
            }
        }
    }
}

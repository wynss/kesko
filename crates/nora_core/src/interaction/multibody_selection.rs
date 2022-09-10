use bevy::prelude::*;

use nora_object_interaction::{
    event::{InteractionEvent, SelectEvent},
    interaction::Select
};
use nora_physics::multibody::{
    MultiBodyChild, MultibodyRoot
};
use crate::interaction::groups::GroupDynamic;

// Event for when a multibody has been selected/deselected
// the entity contained in the event is the root of the multibody
pub enum MultibodySelectionEvent {
    Selected(Entity),
    Deselected(Entity)
}


/// System that handles selection/deselection of multibodies.
/// Also makes sure that only one multibody/singlebody can be selected at a time.
#[allow(clippy::type_complexity)]
pub fn multibody_selection_system(
    mut interaction_event_reader: EventReader<InteractionEvent>,
    mut select_event_writer: EventWriter<SelectEvent>,
    mut multibody_select_event_writer: EventWriter<MultibodySelectionEvent>,
    root_query: Query<(Entity, &MultibodyRoot, &Select<GroupDynamic>), With<MultibodyRoot>>,
    child_query: Query<(&MultiBodyChild, &Select<GroupDynamic>), With<MultiBodyChild>>,
    non_multi: Query<(Entity, &Select<GroupDynamic>), (Without<MultibodyRoot>, Without<MultiBodyChild>)>
) {
    
    for event in interaction_event_reader.iter() {

        // Get the entity that was selected/deselected
        let (entity, should_select) = match event {
            InteractionEvent::Selected(entity) => (Some(entity), true),
            InteractionEvent::Deselected(entity) => (Some(entity), false),
            _ => (None, false)
        };

        if let Some(entity) = entity {
            
            // get links/entities that possibly should be updated and the multibody root entity 
            let (links, root_entity) = if let Ok((root_entity, multi_root, _)) = root_query.get(*entity) {
                // send event that a multibody root has been selected or deselected
                match should_select {
                    true => multibody_select_event_writer.send(MultibodySelectionEvent::Selected(root_entity)),
                    false => multibody_select_event_writer.send(MultibodySelectionEvent::Deselected(root_entity))
                }
                (Some(&multi_root.joints), Some(root_entity))
            } else if let Ok((multi_child, _)) = child_query.get(*entity) {
                (Some(&multi_child.joints), Some(multi_child.root))
            } else {
                // the event was from a singlebody, pretend that it is a multibody with just a root
                (None, Some(*entity))
            };

            if let Some(links) = links {

                // send events to select/deselect the other entities in the multibody
                let mut events = Vec::<SelectEvent>::new();
                for entity in links.values() {
                    if let Ok((_, _, select)) = root_query.get(*entity) {
                        if !select.selected && should_select {
                            events.push(SelectEvent::Select(*entity));
                        } else if select.selected && !should_select {
                            events.push(SelectEvent::Deselect(*entity));
                        }
                    } else if let Ok((_, select)) = child_query.get(*entity) {
                        if !select.selected && should_select {
                            events.push(SelectEvent::Select(*entity));
                        } else if select.selected && !should_select {
                            events.push(SelectEvent::Deselect(*entity));
                        }
                    }
                }

                // send events to select or deselect the other bodies in the multibody
                select_event_writer.send_batch(events.drain(..));
            }

            // handle deselection the previous selected single body or multibody
            let root_entity = root_entity.expect("We should have a root entity");

            // check if there is a multibody selected already, if so trigger deselection
            root_query.for_each(|(entity, _, select)| {
                if select.selected && entity != root_entity && should_select {
                    select_event_writer.send(SelectEvent::Deselect(entity));
                }
            });

            // check if there is a singlebody selected already, if so trigger deselection
            non_multi.for_each(|(entity, select)| {
                if select.selected && should_select && entity != root_entity {
                    select_event_writer.send(SelectEvent::Deselect(entity));
                }
            })
        }
    }
}

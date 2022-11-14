use bevy::prelude::*;

use crate::interaction::groups::GroupDynamic;
use kesko_object_interaction::{
    event::{InteractionEvent, SelectEvent},
    interaction::Select,
};
use kesko_physics::multibody::{MultibodyChild, MultibodyRoot};

// Event for when a multibody has been selected/deselected
// the entity contained in the event is the root of the multibody
#[derive(Debug)]
pub enum MultibodySelectionEvent {
    Selected(Entity),
    Deselected(Entity),
}

/// System that handles selection/deselection of multibodies.
/// Also makes sure that only one multibody/singlebody can be selected at a time.
#[allow(clippy::type_complexity)]
pub fn multibody_selection_system(
    mut interaction_event_reader: EventReader<InteractionEvent>,
    mut select_event_writer: EventWriter<SelectEvent>,
    mut multibody_select_event_writer: EventWriter<MultibodySelectionEvent>,
    multibody_roots: Query<(Entity, &MultibodyRoot, &Select<GroupDynamic>), With<MultibodyRoot>>,
    multibody_childs: Query<(&MultibodyChild, &Select<GroupDynamic>), With<MultibodyChild>>,
    non_multibodies: Query<
        (Entity, &Select<GroupDynamic>),
        (Without<MultibodyRoot>, Without<MultibodyChild>),
    >,
) {
    for event in interaction_event_reader.iter() {
        // Get the entity that was selected/deselected
        let (entity, selected) = match event {
            InteractionEvent::Selected(entity) => (entity, true),
            InteractionEvent::Deselected(entity) => (entity, false),
            _ => continue,
        };

        // get links/entities that possibly should be updated and the multibody root entity
        let (links, root_entity) =
            if let Ok((root_entity, multi_root, _)) = multibody_roots.get(*entity) {
                // send event that a multibody root has been selected or deselected
                match selected {
                    true => multibody_select_event_writer
                        .send(MultibodySelectionEvent::Selected(root_entity)),
                    false => multibody_select_event_writer
                        .send(MultibodySelectionEvent::Deselected(root_entity)),
                }
                (Some(&multi_root.child_map), Some(root_entity))
            } else if let Ok((multi_child, _)) = multibody_childs.get(*entity) {
                (Some(&multi_child.child_map), Some(multi_child.root))
            } else {
                // the event was from a singlebody, pretend that it is a multibody with just a root
                (None, Some(*entity))
            };

        if let Some(links) = links {
            // send events to select/deselect the other entities in the multibody
            let mut events = Vec::<SelectEvent>::new();
            for entity in links.values() {
                if let Ok((_, _, select)) = multibody_roots.get(*entity) {
                    if !select.selected && selected {
                        events.push(SelectEvent::Select(*entity));
                    } else if select.selected && !selected {
                        events.push(SelectEvent::Deselect(*entity));
                    }
                } else if let Ok((_, select)) = multibody_childs.get(*entity) {
                    if !select.selected && selected {
                        events.push(SelectEvent::Select(*entity));
                    } else if select.selected && !selected {
                        events.push(SelectEvent::Deselect(*entity));
                    }
                }
            }

            // send events to select or deselect the other bodies in the multibody
            select_event_writer.send_batch(events.drain(..));
        }

        // handle deselection the previous selected single body or multibody
        let root_entity = root_entity.expect("We should have a root entity");

        // handle deselection when another body has been selected
        // check if there is a multibody selected already, if so trigger deselection
        multibody_roots.for_each(|(entity, _, select)| {
            if select.selected && entity != root_entity && selected {
                select_event_writer.send(SelectEvent::Deselect(entity));
            }
        });

        // check if there is a singlebody selected already, if so trigger deselection
        non_multibodies.for_each(|(entity, select)| {
            if select.selected && selected && entity != root_entity {
                select_event_writer.send(SelectEvent::Deselect(entity));
            }
        })
    }
}

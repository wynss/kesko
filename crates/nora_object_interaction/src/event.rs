use bevy::prelude::*;

use crate::interaction::{Drag, Hover};


pub enum DragEvent {
    Started(Entity),
    Stopped(Entity)
}

pub enum HoverEvent {
    Started(Entity),
    Stopped(Entity)
}

pub enum InteractionEvent {
    Drag(DragEvent),
    Hover(HoverEvent),
    NoInteraction(Entity)
}

#[allow(clippy::type_complexity)]
pub(crate) fn send_events(
    mut event_writer: EventWriter<InteractionEvent>,
    interaction_query: Query<
        (Entity, &Hover, &Drag, ChangeTrackers<Hover>, ChangeTrackers<Drag>),
        Or<(Changed<Hover>, Changed<Drag>)>
    >
) {
    for (e, hover, drag, hover_track, drag_track) in interaction_query.iter() {
        if hover_track.is_changed() || drag_track.is_changed() {
            if drag_track.is_changed() {
                event_writer.send(InteractionEvent::Drag(match drag.dragged {
                    true => DragEvent::Started(e),
                    false => DragEvent::Stopped(e)
                }));
            }
            if hover_track.is_changed() {
                event_writer.send(InteractionEvent::Hover(match hover.hovered {
                    true => HoverEvent::Started(e),
                    false => HoverEvent::Stopped(e)
                }));
            }
            if !hover.hovered && !drag.dragged {
                event_writer.send(InteractionEvent::NoInteraction(e))
            }
        }
    }
}
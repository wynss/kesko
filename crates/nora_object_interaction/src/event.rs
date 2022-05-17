use bevy::prelude::*;


pub enum DragEvent {
    Started(Entity),
    Stopped(Entity)
}

pub enum HoverEvent {
    Started(Entity),
    Stopped(Entity)
}

pub enum InteractionEvent {
    Dragged(DragEvent),
    Hover(HoverEvent)
}
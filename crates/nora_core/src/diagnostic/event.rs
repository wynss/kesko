use bevy::prelude::*;

use nora_object_interaction::event::InteractionEvent;


pub struct DebugEventPlugin;
impl Plugin for DebugEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(debug_events);
    }
}


pub fn debug_events(
    mut event_reader: EventReader<InteractionEvent>
) {
    for interaction_event in event_reader.iter() {
        info!("{:?}", interaction_event);
    }
}
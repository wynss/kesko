mod spawn_component;
pub(crate) mod event;

use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    ui::{
        event::UIEvent,
        spawn_component::SpawnComponent
    }
};

/// Plugin responsible to add all UI components and resources
pub(crate) struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UIEvent>()
            .add_system(handle_ui_components_system)
            .add_system(show_ui_components_system)
            .insert_resource(UIComponents::default());
    }
}

/// UI trait that should be implemented by all ui components that can be rendered
trait UIComponent {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context) -> Option<UIEvent>;
    fn remove(&self) -> bool;
}

/// Resource for holding all ui components
struct UIComponents {
    components: BTreeMap<String, Box<dyn UIComponent + Send + Sync>>
}
impl Default for UIComponents {
    fn default() -> Self {
        Self {
            components: BTreeMap::new()
        } 
    }
}

impl UIComponents {

    /// Shows the active ui components
    pub(crate) fn show(&mut self, ctx: &egui::Context) -> Vec<UIEvent> {

        let Self {
            components,
        } = self;

        let mut events = Vec::<UIEvent>::new();

        for comp in components.values_mut() {
            if let Some(event) = comp.show(ctx) {
                events.push(event);
            }
        }

        events
    }

    /// Clean upcomponents. The components are responsible to indicate themselfs if they should be cleaned up or not.
    pub(crate) fn clean_up(&mut self) {

        let mut to_remove: Vec::<String> = Vec::new();
        for comp in self.components.values() {
            if comp.remove() {
                to_remove.push(comp.name().to_owned());
            }
        }

        for key in to_remove {
            self.components.remove(&key);
        }

    }
}

/// System to handle what components should be shown
fn handle_ui_components_system(
    mut ui_components: ResMut<UIComponents>,
    mut event_reader: EventReader<UIEvent>
) {
    for event in event_reader.iter() {
        match event {
            UIEvent::OpenSpawnWindow => {
                let spawn_component = SpawnComponent::default();
                if !ui_components.components.contains_key(spawn_component.name()) {
                    ui_components.components.insert(spawn_component.name().to_owned(), Box::new(spawn_component));
                }
            }
            _ => { error!("event {:?} not implemented", event)}
        }
    }
}

/// System to actually show the ui components and clean up the components that are stale
fn show_ui_components_system(
    mut ui_event_writer: EventWriter<UIEvent>,
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_components: ResMut<UIComponents>,
) {

    let events = ui_components.show(egui_ctx.ctx_mut());
    if !events.is_empty() {
        ui_event_writer.send_batch(events.into_iter());
    }

    ui_components.clean_up();
}

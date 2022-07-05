pub(crate) mod main_menu;
pub(crate) mod spawn_component;
pub(crate) mod fps_component;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use self::{
    main_menu::MainMenuComponent,
    spawn_component::SpawnComponent,
    fps_component::FPSComponent
};

/// Plugin responsible to add all UI components and resources
pub(crate) struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)

            // setup all components
            .insert_resource(SpawnComponent::default())
            .add_event::<<SpawnComponent as UIComponent>::InEvent>()
            .add_system(update_component_system::<SpawnComponent>)
            .add_system(emit_component_events_system::<SpawnComponent>)

            .insert_resource(MainMenuComponent::default())
            .add_event::<<MainMenuComponent as UIComponent>::InEvent>()
            .add_system(update_component_system::<MainMenuComponent>)
            .add_system(emit_component_events_system::<MainMenuComponent>)

            .insert_resource(FPSComponent::default())
            .add_event::<<FPSComponent as UIComponent>::InEvent>()
            .add_system(update_component_system::<FPSComponent>)
            .add_system(emit_component_events_system::<FPSComponent>);
    }

    fn name(&self) -> &str {
        "ui-plugin"
    }
}

/// UI trait that should be implemented by all ui components that can be rendered
trait UIComponent {

    type InEvent: Send + Sync;
    type OutEvent: Send + Sync;

    fn show(&mut self, ctx: &egui::Context) -> Option<Self::OutEvent>;
    fn remove(&self) -> bool;
    fn toggle_open(&mut self);
    fn handle_event(&mut self, event: &Self::InEvent);
}

trait UIComponentName {
    fn name() -> &'static str;
}

/// System to handle what components should be shown
fn update_component_system<T> (
    mut event_reader: EventReader<T::InEvent>,
    mut comp: ResMut<T>,
) 
where T: 'static + UIComponent + Send + Sync
{
    for event in event_reader.iter() {
        comp.handle_event(event);
    }
}

fn emit_component_events_system<T> (
    mut event_writer: EventWriter<T::OutEvent>,
    mut comp: ResMut<T>,
    mut egui_ctx: ResMut<EguiContext>

) 
where T: 'static + UIComponent + Send + Sync
{
    if let Some(event) = comp.show(egui_ctx.ctx_mut()) {
        event_writer.send(event);
    }
}

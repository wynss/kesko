pub(crate) mod main_menu;
pub(crate) mod spawn_component;
pub(crate) mod fps_component;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};


#[derive(SystemLabel, Debug, PartialEq, Eq, Clone, Hash)]
enum UISystems {
   MainMenu 
}


/// Plugin responsible to add all UI components and resources
pub(crate) struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)

            // core ui
            .add_startup_system(initialize_ui_components_system)

            // add components
            .add_system(main_menu::MainMenuComponent::update_system.label(UISystems::MainMenu))

            .add_event::<spawn_component::SpawnEvent>()
            .add_system(spawn_component::SpawnComponent::update_system.after(UISystems::MainMenu))
            .add_system(spawn_component::SpawnComponent::show_and_send_system)

            .add_event::<fps_component::FPSComponentEvent>()
            .add_system(fps_component::FPSComponent::update_system)
            .add_system(fps_component::FPSComponent::show_and_send_system);
    }

    fn name(&self) -> &str {
        "ui-plugin"
    }
}


/// system to initialize ui components
fn initialize_ui_components_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
) {
    egui_context.as_mut().ctx_mut().set_visuals(egui::Visuals::light());
    commands.spawn().insert(main_menu::MainMenuComponent::default());
    commands.spawn().insert(spawn_component::SpawnComponent::default());
    commands.spawn().insert(fps_component::FPSComponent::default());
}

pub(crate) mod main_menu;
pub(crate) mod spawn_component;
pub mod fps_component;
pub(crate) mod multibody_component;
pub(crate) mod about;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

#[derive(SystemLabel, Debug, PartialEq, Eq, Clone, Hash)]
enum UISystems {
   MainMenu 
}


/// Plugin responsible to add all UI components and resources
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)

            // core ui
            .add_startup_system(initialize_ui_components_system)

            // add components

            // main menu component
            .add_system(main_menu::MainMenuComponent::update_system.label(UISystems::MainMenu))

            .add_event::<about::AboutEvent>()
            .add_system(about::AboutComponent::update_system)

            // spawn model component
            .add_system(spawn_component::SpawnComponent::update_system.after(UISystems::MainMenu))
            .add_system(spawn_component::SpawnComponent::show_and_send_system)

            // FPS component
            .add_event::<fps_component::FPSComponentEvent>()
            .add_system(fps_component::FPSComponent::update_system)
            .add_system(fps_component::FPSComponent::show_and_send_system)

            // Add UI component for multibody interaction
            .add_system(multibody_component::MultibodyUIComponent::show_system);
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
    commands.spawn().insert(about::AboutComponent::default());
    commands.spawn().insert(spawn_component::SpawnComponent::default());
    commands.spawn().insert(fps_component::FPSComponent::default());
    commands.spawn().insert(multibody_component::MultibodyUIComponent::default());
}

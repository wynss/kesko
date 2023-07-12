pub(crate) mod about;
pub mod fps_component;
pub(crate) mod main_menu;
pub(crate) mod multibody_component;
pub(crate) mod spawn_component;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

/// Plugin responsible to add all UI components and resources
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            // core ui
            .add_startup_system(initialize_ui_components_system)
            // add components
            // main menu component
            .add_systems(
                Update,
                (
                    main_menu::MainMenuComponent::update_system,
                    spawn_component::SpawnComponent::update_system,
                    about::AboutComponent::update_system,
                    fps_component::FPSComponent::update_system,
                )
                    .chain(),
            )
            .add_event::<about::AboutEvent>()
            .add_systems(
                Update,
                spawn_component::SpawnComponent::show_and_send_system,
            )
            .add_event::<fps_component::FPSComponentEvent>()
            .add_systems(Update, fps_component::FPSComponent::show_and_send_system)
            .add_systems(
                Update,
                multibody_component::MultibodyUIComponent::show_system,
            );
    }

    fn name(&self) -> &str {
        "ui-plugin"
    }
}

/// system to initialize ui components
fn initialize_ui_components_system(mut commands: Commands, mut egui_context: EguiContexts) {
    egui_context.ctx_mut().set_visuals(egui::Visuals::light());
    commands.spawn((
        main_menu::MainMenuComponent::default(),
        about::AboutComponent::default(),
        spawn_component::SpawnComponent::default(),
        fps_component::FPSComponent::default(),
        multibody_component::MultibodyUIComponent::default(),
    ));
}

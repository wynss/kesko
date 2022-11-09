use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub(crate) enum AboutEvent {
    Open,
}

#[derive(Component, Default)]
pub(crate) struct AboutComponent {
    open: bool,
}
impl AboutComponent {
    pub(crate) fn update_system(
        mut egui_context: ResMut<EguiContext>,
        mut event_reader: EventReader<AboutEvent>,
        mut comp: Query<&mut Self>,
    ) {
        let mut comp = comp.get_single_mut().unwrap();
        for event in event_reader.iter() {
            match event {
                AboutEvent::Open => {
                    comp.open = true;
                }
            }
        }

        egui::Window::new("About")
            .open(&mut comp.open)
            .show(egui_context.ctx_mut(), |ui| {
                // TODO: Move this when versioning is addressed
                ui.label("Version: 0.1-alpha");
            });
    }
}

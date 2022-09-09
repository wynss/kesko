use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use nora_physics::multibody::MultibodyRoot;

use crate::interaction::multibody_selection::MultibodySelectionEvent;
use nora_object_interaction::event::SelectEvent;


/// UI Component that handles showing multibody information and also controlling them
#[derive(Component, Default)]
pub(crate) struct MultibodyMenuComponent {
    open: bool,
    body_name: String,
    root_entity: Option<Entity>
}

impl MultibodyMenuComponent {

    pub(crate) fn show_system(
        mut multibody_select_event_reader: EventReader<MultibodySelectionEvent>,
        mut select_event_writer: EventWriter<SelectEvent>,
        mut egui_context: ResMut<EguiContext>,
        mut comp: Query<&mut Self>,
        body_query: Query<&MultibodyRoot>
    ) {

        // get a reference to 'self'
        let mut comp = comp.get_single_mut().expect("We should have a component");

        // read a possible event that a multibody has been selected/deselected
        if !multibody_select_event_reader.is_empty() {
            if let Some(event) = multibody_select_event_reader.iter().last() {
                match event {
                    MultibodySelectionEvent::Selected(root_entity) => {
                        comp.root_entity = Some(*root_entity);
                        comp.open = true;
                        if let Ok(root) = body_query.get(*root_entity) {
                            comp.body_name = root.name.clone();
                        }
                    },
                    MultibodySelectionEvent::Deselected(entity) => {
                        if Some(*entity) == comp.root_entity {
                            comp.root_entity = None;
                            comp.open = false;
                        }
                    }
                }
            }
        }
        
        // run ui logic
        comp.show(egui_context.ctx_mut(), &mut select_event_writer);
    }

    fn show(&mut self, ctx: &egui::Context, select_event_writer: &mut EventWriter<SelectEvent>) {
        egui::Window::new("Multibody View")
            .open(&mut self.open)
            .resizable(true)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading(&self.body_name);
                    if ui.button("Close").clicked() {
                        if let Some(entity) = self.root_entity {
                            select_event_writer.send(SelectEvent::Deselect(entity))
                        }
                    }
                });
            });
    }
}

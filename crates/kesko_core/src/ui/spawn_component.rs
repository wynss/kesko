use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::models::Model;


#[derive(Component)]
/// UI component for spawning default models
pub(crate) struct SpawnComponent {
    /// World position
    x: f32, y: f32, z: f32,
    /// Color
    color: egui::Color32,
    /// Model to spawn
    model: Model,
    /// If the component is open or not
    open: bool
}

impl Default for SpawnComponent {
    fn default() -> Self {
        Self { 
            x: 0.0, 
            y: 5.0, 
            z: 0.0, 
            color: egui::Color32::from_rgb(255, 0, 0),
            open: false,
            model: Model::Sphere
        }
    } 
}

impl SpawnComponent {

    pub(crate) fn update_system(
        mut spawn_event_reader: EventReader<SpawnEvent>,
        mut comp: Query<&mut Self>
    ) {
        let mut comp = comp.get_single_mut().unwrap();

        for event in spawn_event_reader.iter() {
            if let SpawnEvent::OpenWindow = event {
                comp.open = true;
            }
        }
    }

    pub(crate) fn show_and_send_system(
        mut egui_context: ResMut<EguiContext>,
        mut spawn_event_writer: EventWriter<SpawnEvent>,
        mut comp: Query<&mut Self>
    ) {
        let mut comp = comp.get_single_mut().unwrap();
        comp.show(egui_context.ctx_mut(), &mut spawn_event_writer);
    }

    fn show(&mut self, ctx: &egui::Context, spawn_event_writer: &mut EventWriter<SpawnEvent>) {
        let Self {
            x, 
            y, 
            z,
            color,
            open,
            model
        } = self;

        egui::Window::new("Spawn model").open(open).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("Model")
                        .selected_text(format!("{:?}", model))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(model, Model::Sphere, Model::Sphere.name());
                            ui.selectable_value(model, Model::Car, Model::Car.name());
                            ui.selectable_value(model, Model::Snake, Model::Snake.name());
                            ui.selectable_value(model, Model::Spider, Model::Spider.name());
                            ui.selectable_value(model, Model::Wheely, Model::Wheely.name());
                    });
                });
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(x).speed(0.1).prefix("x: ")
                    );
                    ui.add(
                        egui::DragValue::new(y).speed(0.1).prefix("y: ")
                    );
                    ui.add(
                        egui::DragValue::new(z).speed(0.1).prefix("z: ")
                    );
                    ui.label("World position");
                });
                ui.horizontal(|ui| {
                    ui.color_edit_button_srgba(color);
                    ui.label("Color");
                });

                ui.separator();

                if ui.button("Spawn").clicked() {
                    spawn_event_writer.send( SpawnEvent::Spawn {
                        model: model.clone(),
                        transform: Transform::from_xyz(*x, *y, *z),
                        color: Color::rgb_u8(color.r(), color.g(), color.b())
                    });
                    ui.close_menu();
                }
            });
        });
    }

}

pub(crate) enum SpawnEvent {
    OpenWindow,
    Spawn {
        model: Model,
        transform: Transform,
        color: Color
    }
}

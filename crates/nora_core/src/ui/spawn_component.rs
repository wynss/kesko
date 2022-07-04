use bevy::prelude::*;
use bevy_egui::egui;

use super::event::UIEvent;
use crate::models::Model;

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
            open: true,
            model: Model::Spider
        }
    } 
}

impl super::UIComponent for SpawnComponent {
    fn name(&self) -> &'static str {
        "spawn component"
    }

    fn show(&mut self, ctx: &egui::Context) -> Option<UIEvent> {
        let Self {
            x, 
            y, 
            z,
            color,
            open,
            model
        } = self;

        let mut event: Option<UIEvent> = None;

        egui::Window::new("Spawn model").open(open).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("Model")
                        .selected_text(format!("{:?}", model))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(model, Model::Car, Model::Car.name());
                            ui.selectable_value(model, Model::Snake, Model::Snake.name());
                            ui.selectable_value(model, Model::Spider, Model::Spider.name());
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
                    event = Some(UIEvent::SpawnModel {
                        model: model.clone(),
                        transform: Transform::from_xyz(*x, *y, *z),
                        color: Color::rgb_u8(color.r(), color.g(), color.b())
                    });
                    ui.close_menu();
                }
            });
        });

        event
    }

    fn remove(&self) -> bool {
        if !self.open {
            return true;
        }

        false
    }
}

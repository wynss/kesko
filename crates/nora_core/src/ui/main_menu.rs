use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::ui::{
    spawn_component::SpawnEvent,
    fps_component::FPSComponentEvent,
    about::AboutEvent
};


#[derive(Default)]
pub struct OccupiedScreenSpaceByMenus {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32
}

#[derive(Component)]
pub(crate) struct MainMenuComponent {
    hide_panels: bool
}

impl MainMenuComponent {

    pub(crate) fn update_system(
        mut egui_context: ResMut<EguiContext>,
        mut about_event_writer: EventWriter<AboutEvent>,
        mut spawn_event_writer: EventWriter<SpawnEvent>,
        mut fps_event_writer: EventWriter<FPSComponentEvent>,
        mut comp: Query<&mut Self>
    ) {
        comp.get_single_mut().unwrap().show_and_send_system(
            egui_context.ctx_mut(), 
            &mut about_event_writer,
            &mut spawn_event_writer, 
            &mut fps_event_writer
        );
    }

    fn show_and_send_system(&mut self, 
        ctx: &egui::Context, 
        about_event_writer: &mut EventWriter<AboutEvent>,
        spawn_event_writer:&mut EventWriter<SpawnEvent>, 
        fps_event_writer: &mut EventWriter<FPSComponentEvent>
    ) {

        egui::TopBottomPanel::top("top_menu")
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {

                    egui::widgets::global_dark_light_mode_switch(ui);

                    ui.menu_button("Nora", |ui| {
                        if ui.button("About").clicked() {
                            about_event_writer.send(AboutEvent::Open);
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("View", |ui| {
                        if ui.button("Show/Hide Panels").clicked() {
                            self.hide_panels = !self.hide_panels;
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Spawn", |ui| {
                        if ui.button("Model").clicked() {
                            spawn_event_writer.send(SpawnEvent::OpenWindow);
                            ui.close_menu();
                        };
                    });

                    ui.menu_button("Diagnostics", |ui| {
                        if ui.button("FPS").clicked() {
                            fps_event_writer.send(FPSComponentEvent::Open);
                            ui.close_menu();
                        }
                    });
                });
            });

        if !self.hide_panels {

            egui::SidePanel::left("left_side_menu")
                .resizable(true)
                .default_width(200.0)
                .max_width(400.0)
                .show(ctx, |ui| {
                    ui.heading("Left Menu");
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
                });

            egui::SidePanel::right("right_side_menu")
                .resizable(true)
                .default_width(200.0)
                .max_width(400.0)
                .show(ctx, |ui| {
                    ui.heading("Right Menu");
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
                });

            egui::TopBottomPanel::bottom("bottom_menu")
                .resizable(true)
                .default_height(100.0)
                .max_height(200.0)
                .show(ctx, |ui| {
                    ui.heading("Bottom Menu");
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
                });
        }
    }
}

impl Default for MainMenuComponent {
    fn default() -> Self {
        MainMenuComponent {
            hide_panels: true
        }
    }
}

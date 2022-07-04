use bevy_egui::egui;

use crate::ui::event::UIEvent;


#[derive(Default)]
pub struct OccupiedScreenSpaceByMenus {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32
}

pub(crate) struct MainMenuComponent {
    hide_panels: bool
}

impl super::UIComponent for MainMenuComponent {
    fn name(&self) -> &'static str {
        "main-menu-component"
    }

    fn show(&mut self, ctx: &egui::Context) -> Option<UIEvent> {

        let mut event: Option<UIEvent> = None;

        egui::TopBottomPanel::top("top_menu")
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {

                    egui::widgets::global_dark_light_mode_switch(ui);

                    ui.menu_button("View", |ui| {
                        if ui.button("Show/Hide Panels").clicked() {
                            self.hide_panels = !self.hide_panels;
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Spawn", |ui| {
                        if ui.button("Model").clicked() {
                            event = Some(UIEvent::OpenSpawnWindow);
                            ui.close_menu();
                        };
                    });

                    ui.menu_button("Diagnostics", |ui| {
                        todo!("Implement");
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

        event
        
    }

    fn remove(&self) -> bool {
        false
    }
}

impl Default for MainMenuComponent {
    fn default() -> Self {
        MainMenuComponent {
            hide_panels: true
        }
    }
}

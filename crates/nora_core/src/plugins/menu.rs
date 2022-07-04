use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::ui::event::UIEvent;


#[derive(Default)]
pub struct OccupiedScreenSpaceByMenus {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32
}

struct MenusContext {
    should_hide: bool
}

impl Default for MenusContext {
    fn default() -> Self {
        MenusContext {
            should_hide: true
        }
    }
}


#[derive(Default)]
pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(update_menus)
            .init_resource::<MenusContext>()
            .init_resource::<OccupiedScreenSpaceByMenus>();
    }

    fn name(&self) -> &str {
        "Menu Plugin"
    }
}

fn update_menus(
    mut event_writer: EventWriter<UIEvent>,
    mut occupied_screen_space_by_menus: ResMut<OccupiedScreenSpaceByMenus>,
    mut menus_ctx: ResMut<MenusContext>,
    mut egui_ctx: ResMut<EguiContext>
) {

    occupied_screen_space_by_menus.top = egui::TopBottomPanel::top("top_menu")
        .show(egui_ctx.ctx_mut(), |ui| {
            egui::menu::bar(ui, |ui| {

                egui::widgets::global_dark_light_mode_switch(ui);

                ui.menu_button("View", |ui| {
                    if ui.button("Show/Hide menus").clicked() {
                        menus_ctx.should_hide = !menus_ctx.should_hide;
                        if menus_ctx.should_hide {
                            occupied_screen_space_by_menus.top = 0.0;
                            occupied_screen_space_by_menus.right = 0.0;
                            occupied_screen_space_by_menus.bottom = 0.0;
                            occupied_screen_space_by_menus.left = 0.0;
                        }
                        ui.close_menu();
                    }
                });

                ui.menu_button("Spawn", |ui| {
                    if ui.button("Model").clicked() {
                        event_writer.send(UIEvent::OpenSpawnWindow);
                        ui.close_menu();
                    };
                });

                ui.menu_button("Diagnostics", |ui| {
                    todo!("Implement");
                });
            });
        })
        .response
        .rect
        .top();

    if !menus_ctx.should_hide {

        occupied_screen_space_by_menus.left = egui::SidePanel::left("left_side_menu")
            .resizable(true)
            .default_width(200.0)
            .max_width(400.0)
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.heading("Left Menu");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .left();

        occupied_screen_space_by_menus.right = egui::SidePanel::right("right_side_menu")
            .resizable(true)
            .default_width(200.0)
            .max_width(400.0)
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.heading("Right Menu");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .right();

        occupied_screen_space_by_menus.bottom = egui::TopBottomPanel::bottom("bottom_menu")
            .resizable(true)
            .default_height(100.0)
            .max_height(200.0)
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.heading("Bottom Menu");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .bottom();
    }
}

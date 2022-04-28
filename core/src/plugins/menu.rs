use bevy::{
    app::{App, Plugin},
    ecs::change_detection::ResMut,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};


#[derive(Default)]
pub struct OccupiedScreenSpaceByMenus {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32
}

#[derive(Default)]
pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(side_menus)
            .init_resource::<OccupiedScreenSpaceByMenus>();
    }

    fn name(&self) -> &str {
        "Menu Plugin"
    }
}

fn side_menus(
    mut occupied_screen_space_by_menus: ResMut<OccupiedScreenSpaceByMenus>,
    mut egui_ctx: ResMut<EguiContext>
) {

    occupied_screen_space_by_menus.top = egui::TopBottomPanel::top("top_menu")
        .show(egui_ctx.ctx_mut(), |ui| {
            egui::menu::bar(ui, |ui| {

                egui::widgets::global_dark_light_mode_switch(ui);

                ui.menu_button("File", |_ui| {

                });
            });
        })
        .response
        .rect
        .top();

    occupied_screen_space_by_menus.left =egui::SidePanel::left("left_side_menu")
        .resizable(true)
        .default_width(200.0)
        .max_width(400.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            if ui.button("Load URDF").clicked() {

            }

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

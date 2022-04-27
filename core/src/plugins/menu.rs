use bevy::{
    app::{App, Plugin},
    ecs::change_detection::ResMut,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};


#[derive(Default)]
pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(side_menus);
    }

    fn name(&self) -> &str {
        "Menu Plugin"
    }
}

fn side_menus(
    mut egui_ctx: ResMut<EguiContext>
) {

    egui::TopBottomPanel::top("top_menu")
        .show(egui_ctx.ctx_mut(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {

                });
            });
        });

    egui::SidePanel::left("left_side_menu")
        .resizable(true)
        .default_width(200.0)
        .max_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            if ui.button("Load URDF").clicked() {

            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

    egui::SidePanel::right("right_side_menu")
        .resizable(true)
        .default_width(200.0)
        .max_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Right Menu");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

    egui::TopBottomPanel::bottom("bottom_menu")
        .resizable(true)
        .default_height(100.0)
        .max_height(100.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Bottom Menu");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
}

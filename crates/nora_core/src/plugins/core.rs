use bevy::{
    core_pipeline::clear_color::ClearColor,
    render::{color::Color, view::Msaa},
    app::{App, Plugin},
    window::WindowDescriptor, DefaultPlugins
};
use crate::{interaction::{
    groups::GroupStatic,
    vertical_marker::{
        update_vertical_marker_pos_system,
        handle_vertical_marker_spawning
    }
}, models::spawn_model_system};


#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("FFFFFF").unwrap()))
            .insert_resource(WindowDescriptor {
                title: String::from("Nora 0.1-alpha"),
                width: 1280.0,
                height: 720.0,
                fit_canvas_to_parent: true,
                canvas: Some("#nora-wasm".to_string()),
                ..Default::default()
            })
            .insert_resource(Msaa { samples: 4 })
            .add_plugins(DefaultPlugins)
            .add_system(handle_vertical_marker_spawning::<GroupStatic>)
            .add_system(update_vertical_marker_pos_system::<GroupStatic>)
            .add_system(spawn_model_system)
            .add_system(bevy::window::close_on_esc);
    }
}

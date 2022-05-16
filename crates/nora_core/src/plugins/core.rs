use bevy::{
    core_pipeline::ClearColor,
    render::{color::Color, view::Msaa},
    app::{App, Plugin},
    window::WindowDescriptor
};
use bevy::window::PresentMode;


#[derive(Default)]
pub(crate) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("FFFFFF").unwrap()))
            .insert_resource(WindowDescriptor {
                present_mode: PresentMode::Mailbox,
                title: String::from("Nora 0.1-alpha"),
                width: 1280.0,
                height: 720.0,
                ..Default::default()
            })
            .insert_resource(Msaa { samples: 4 })
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}
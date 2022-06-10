use bevy::{
    core_pipeline::ClearColor,
    render::{color::Color, view::Msaa},
    app::{App, Plugin},
    window::WindowDescriptor
};
use bevy::window::PresentMode;
use crate::{
    event::{SystemEvent, handle_system_events},
    interaction::{
        groups::GroupStatic,
        vertical_marker::{
            update_vertical_marker_pos_system,
            handle_vertical_marker_spawning
        }
    }
};


#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClearColor(Color::hex("FFFFFF").unwrap()))
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Mailbox,
            title: String::from("Nora 0.1-alpha"),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })

        // System events
        .add_event::<SystemEvent>()
        .add_system(handle_system_events)

        // vertical marker
        // todo: Should not be here
        .add_system(handle_vertical_marker_spawning::<GroupStatic>)
        .add_system(update_vertical_marker_pos_system::<GroupStatic>)
        .add_system(bevy::input::system::exit_on_esc_system);
    }
}

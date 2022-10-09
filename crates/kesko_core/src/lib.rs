pub mod orbit_camera;
pub mod cursor_tracking;
pub mod bundle;
pub mod shape;
pub mod transform;
pub mod interaction;
pub mod controller;
pub mod event;

use bevy::prelude::*;

use kesko_physics::event::PhysicStateEvent;

use bevy::{
    core_pipeline::clear_color::ClearColor,
    render::{color::Color, view::Msaa},
    app::{App, Plugin},
    window::{
        WindowDescriptor,
        WindowPosition,
        MonitorSelection
    }, 
    DefaultPlugins,
};
use crate::{
    interaction::{
        groups::GroupStatic,
        vertical_marker::{
            update_vertical_marker_pos_system,
            handle_vertical_marker_spawning
        },
        multibody_selection::{
            multibody_selection_system, 
            MultibodySelectionEvent
        }
    }
};


#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("FFFFFF").unwrap()))
            .insert_resource(WindowDescriptor {
                title: String::from("Kesko 0.1-alpha"),
                width: 1920.0,
                height: 1080.0,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                fit_canvas_to_parent: true,
                canvas: Some("#kesko-wasm".to_string()),
                ..Default::default()
            })
            .insert_resource(Msaa { samples: 4 })
            
            .add_plugins(DefaultPlugins)

            // vertical marker systems
            .add_system(handle_vertical_marker_spawning::<GroupStatic>)
            .add_system(update_vertical_marker_pos_system::<GroupStatic>)

            // physics related
            .add_system(change_physic_state)

            // multibody selection systems and events
            .add_system(multibody_selection_system)
            .add_event::<MultibodySelectionEvent>()

            // close on ESC
            .add_system(bevy::window::close_on_esc);
    }
}


pub fn change_physic_state(
    mut keys: ResMut<Input<KeyCode>>,
    mut event_writer: EventWriter<PhysicStateEvent>
) {
    if keys.just_pressed(KeyCode::Space) {
        event_writer.send(PhysicStateEvent::ToggleRunPause);
        keys.reset(KeyCode::Space);
    }
}

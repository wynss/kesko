use bevy::{
    core_pipeline::clear_color::ClearColor,
    render::{color::Color, view::Msaa},
    app::{App, Plugin},
    window::WindowDescriptor, DefaultPlugins
};
use kesko_core::{
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
use kesko_models::spawn_model_system;


#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("FFFFFF").unwrap()))
            .insert_resource(WindowDescriptor {
                title: String::from("Kesko 0.1-alpha"),
                width: 1280.0,
                height: 720.0,
                fit_canvas_to_parent: true,
                canvas: Some("#kesko-wasm".to_string()),
                ..Default::default()
            })
            .insert_resource(Msaa { samples: 4 })
            
            .add_plugins(DefaultPlugins)

            // vertical marker systems
            .add_system(handle_vertical_marker_spawning::<GroupStatic>)
            .add_system(update_vertical_marker_pos_system::<GroupStatic>)

            // multibody selection systems and events
            .add_system(multibody_selection_system)
            .add_event::<MultibodySelectionEvent>()

            .add_system(spawn_model_system)

            // close on ESC
            .add_system(bevy::window::close_on_esc);
    }
}

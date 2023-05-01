pub mod bundle;
pub mod controller;
pub mod cursor_tracking;
pub mod event;
pub mod interaction;
pub mod orbit_camera;
pub mod shape;
pub mod transform;

use bevy::{log::LogPlugin, prelude::*};
use kesko_physics::event::PhysicRequestEvent;

use crate::{
    cursor_tracking::GrabablePlugin,
    interaction::{
        groups::{GroupDynamic, GroupStatic},
        multibody_selection::{multibody_selection_system, MultibodySelectionEvent},
        vertical_marker::{handle_vertical_marker_spawning, update_vertical_marker_pos_system},
    },
};
use bevy::{
    app::{App, Plugin},
    core_pipeline::clear_color::ClearColor,
    log::Level,
    render::{color::Color, view::Msaa},
    window::{MonitorSelection, WindowDescriptor, WindowPlugin, WindowPosition},
    DefaultPlugins,
};

#[derive(Default)]
pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("FFFFFF").unwrap()))
            .insert_resource(Msaa { samples: 4 })
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        window: WindowDescriptor {
                            title: String::from("Kesko 0.0.4"),
                            width: 1920.0,
                            height: 1080.0,
                            position: WindowPosition::Centered(MonitorSelection::Primary),
                            fit_canvas_to_parent: true,
                            canvas: Some("#kesko-wasm".to_string()),
                            ..Default::default()
                        },
                        add_primary_window: true,
                        exit_on_all_closed: true,
                        close_when_requested: true,
                    })
                    .set(LogPlugin {
                        level: Level::INFO,
                        ..default()
                    }),
            )
            .add_plugin(GrabablePlugin::<GroupDynamic>::default())
            // vertical marker systems
            .add_system(handle_vertical_marker_spawning::<GroupStatic>)
            .add_system(update_vertical_marker_pos_system::<GroupStatic>)
            // physics related
            .add_system(change_physic_state_on_space)
            // multibody selection systems and events
            .add_system(multibody_selection_system)
            .add_event::<MultibodySelectionEvent>()
            // simulator system events
            .add_event::<event::SimulatorRequestEvent>()
            .add_event::<event::SimulatorResponseEvent>()
            .add_system_set_to_stage(
                CoreStage::Last,
                SystemSet::new()
                    .with_system(event::handle_system_events)
                    .with_system(event::handle_serializable_state_request)
                    .with_system(event::handle_motor_command_requests),
            );
    }
}

pub struct CoreHeadlessPlugin;
impl Plugin for CoreHeadlessPlugin {
    fn build(&self, app: &mut App) {
        // Bevy plugins
        app.add_plugins(
            DefaultPlugins
                .build()
                .disable::<bevy::winit::WinitPlugin>()
                .set(LogPlugin {
                    level: Level::INFO,
                    ..default()
                }),
        )
        .set_runner(headless_runner)
        // Simulator system events
        .add_event::<event::SimulatorRequestEvent>()
        .add_event::<event::SimulatorResponseEvent>()
        .add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::new()
                .with_system(event::handle_system_events)
                .with_system(event::handle_serializable_state_request)
                .with_system(event::handle_motor_command_requests),
        );
    }
}

pub fn change_physic_state_on_space(
    mut keys: ResMut<Input<KeyCode>>,
    mut event_writer: EventWriter<PhysicRequestEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        event_writer.send(PhysicRequestEvent::TogglePhysics);
        keys.reset(KeyCode::Space);
    }
}

fn headless_runner(mut app: App) {
    loop {
        app.update();
    }
}

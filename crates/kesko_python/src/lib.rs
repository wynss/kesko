use std::collections::BTreeMap;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use pyo3::prelude::*;

use kesko_core::event::{SimulatorRequestEvent, SimulatorResponseEvent};
use kesko_models::{car::CarPlugin, wheely::WheelyPlugin, Model, SpawnEvent};
use kesko_physics::{
    event::{collision::CollisionEvent, PhysicRequestEvent, PhysicResponseEvent},
    joint::{JointMotorEvent, MotorCommand},
};
use kesko_plugins::CorePlugins;
use kesko_plugins::HeadlessRenderPlugins;

#[pymodule]
fn pykesko(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Kesko>()?;
    m.add_class::<Model>()?;
    Ok(())
}
#[pyclass(unsendable)]
pub struct Kesko {
    app: App,
}

#[pymethods]
impl Kesko {
    #[new]
    pub fn new() -> Self {
        Self { app: App::new() }
    }

    pub fn init_default(&mut self) {
        self.app
            .add_plugins(CorePlugins)
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(CarPlugin)
            .add_plugin(WheelyPlugin)
            .add_startup_system(start_scene);
    }

    pub fn init_headless(&mut self) {
        self.app
            .add_plugins(HeadlessRenderPlugins::default())
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(start_scene);
    }

    pub fn spawn(&mut self, model: Model, position: Vec<f32>, color: Vec<f32>) {
        self.app.world.send_event::<SpawnEvent>(SpawnEvent::Spawn {
            model,
            transform: Transform::from_xyz(position[0], position[1], position[2]),
            color: Color::Rgba {
                red: color[0],
                green: color[1],
                blue: color[2],
                alpha: 1.0,
            },
        })
    }

    pub fn despawn(&mut self, body_id: u64) {
        self.app
            .world
            .send_event(PhysicRequestEvent::DespawnBody(body_id));
    }

    pub fn despawn_all(&mut self) {
        self.app.world.send_event(PhysicRequestEvent::DespawnAll);
    }

    pub fn get_physics_events(&mut self) -> PyResult<Option<String>> {
        let events = self.app.world.resource_mut::<Events<PhysicResponseEvent>>();
        if events.is_empty() {
            return Ok(None);
        }

        let mut reader = events.get_reader();
        let events_vec = reader
            .iter(&events)
            .cloned()
            .collect::<Vec<PhysicResponseEvent>>();
        Ok(Some(
            serde_json::to_string_pretty(&events_vec).expect("Could not serialize events"),
        ))
    }

    pub fn get_collisions(&mut self) -> PyResult<Option<String>> {
        let events = self.app.world.resource_mut::<Events<CollisionEvent>>();
        if events.is_empty() {
            return Ok(None);
        }

        let mut reader = events.get_reader();
        let events_vec = reader
            .iter(&events)
            .cloned()
            .collect::<Vec<CollisionEvent>>();
        Ok(Some(
            serde_json::to_string_pretty(&events_vec).expect("Could not serialize events"),
        ))
    }

    pub fn step(&mut self) {
        self.app
            .world
            .send_event::<SimulatorRequestEvent>(SimulatorRequestEvent::GetState);
        self.app.update();
    }

    pub fn apply_motor_commands(&mut self, command: BTreeMap<u64, f32>) {
        let world = &mut self.app.world;
        for (joint_id, val) in command.iter() {
            let entity = Entity::from_bits(*joint_id);
            world.send_event::<JointMotorEvent>(JointMotorEvent {
                entity,
                command: MotorCommand::PositionRevolute {
                    position: *val,
                    stiffness: None,
                    damping: None,
                },
            });
        }
    }

    pub fn get_multibody_state(&mut self) -> PyResult<Option<String>> {
        let events = self
            .app
            .world
            .resource_mut::<Events<SimulatorResponseEvent>>();
        let mut reader = events.get_reader();
        if reader.is_empty(&events) {
            return Ok(None);
        }

        for event in reader.iter(&events) {
            if let SimulatorResponseEvent::MultibodyStates(states) = event {
                return Ok(Some(serde_json::to_string_pretty(states).unwrap()));
            }
        }

        Ok(None)
    }

    pub fn start_physics(&mut self) {
        self.app.world.send_event(PhysicRequestEvent::RunPhysics);
    }

    pub fn stop_physics(&mut self) {
        self.app.world.send_event(PhysicRequestEvent::PausePhysics);
    }

    pub fn close(&mut self) {
        self.app.world.send_event(bevy::app::AppExit);
    }
}

impl Default for Kesko {
    fn default() -> Self {
        Self::new()
    }
}

fn start_scene(mut commands: Commands) {
    // Light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}

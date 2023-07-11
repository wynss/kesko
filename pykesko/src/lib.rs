use std::collections::BTreeMap;

use bevy::log::Level;
use bevy::{prelude::*, utils::hashbrown::HashMap};
use phf::phf_map;
use pyo3::prelude::*;

use kesko::core::event::{SimulatorRequestEvent, SimulatorResponseEvent};
use kesko::models::{car::CarPlugin, wheely::WheelyPlugin, Model, SpawnEvent};
use kesko::physics::{
    event::{collision::CollisionEvent, PhysicRequestEvent, PhysicResponseEvent},
    joint::{JointMotorEvent, MotorCommand},
};
use kesko::plugins::{CorePlugins, HeadlessRenderPlugins, UIPlugin};
use kesko::tcp::TcpPlugin;
use kesko_urdf::UrdfPlugin;
use placeholder_box::PlaceholderBoxPlugin;

static PYTHON_LOG_TO_BEVY_LOG_LEVEL: phf::Map<i32, Level> = phf_map! {
    10i32 => Level::DEBUG,
    20i32 => Level::INFO,
    30i32 => Level::WARN,
    40i32 => Level::ERROR,
};

#[pymodule]
fn pykesko(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<KeskoApp>()?;
    m.add_class::<Model>()?;
    m.add_function(wrap_pyfunction!(run_kesko_tcp, m)?)?;
    Ok(())
}

/// Function to start Kesko with tcp communication
#[pyfunction]
fn run_kesko_tcp(window: bool, log_level: i32) {
    let bevy_log_level = PYTHON_LOG_TO_BEVY_LOG_LEVEL.get(&log_level).unwrap();

    if window {
        App::new()
            .add_plugins(CorePlugins {
                log_level: *bevy_log_level,
            })
            .add_plugin(CarPlugin)
            .add_plugin(WheelyPlugin)
            .add_plugin(UrdfPlugin)
            .add_plugin(PlaceholderBoxPlugin)
            .add_plugin(TcpPlugin)
            .add_startup_system(start_scene)
            .run();
    } else {
        App::new()
            .add_plugins(HeadlessRenderPlugins::default())
            .add_plugin(TcpPlugin)
            .add_startup_system(start_scene)
            .run();
    }
}

/// Hold an instance to the Kesko app
#[pyclass(unsendable)]
pub struct KeskoApp {
    app: App,
    log_level: Level,
}

#[pymethods]
impl KeskoApp {
    #[new]
    pub fn new(log_level: i32) -> Self {
        let bevy_log_level = PYTHON_LOG_TO_BEVY_LOG_LEVEL.get(&log_level).unwrap();
        Self {
            app: App::new(),
            log_level: *bevy_log_level,
        }
    }

    pub fn init_default(&mut self) {
        self.app
            .add_plugins(
                CorePlugins {
                    log_level: self.log_level,
                }
                .build()
                .disable::<UIPlugin>(),
            )
            .add_plugin(CarPlugin)
            .add_plugin(WheelyPlugin)
            .add_plugin(UrdfPlugin)
            .add_plugin(PlaceholderBoxPlugin)
            .add_startup_system(start_scene);
        self.app.setup();
    }

    pub fn init_headless(&mut self) {
        self.app
            .add_plugins(HeadlessRenderPlugins::default())
            .add_startup_system(start_scene);
    }

    pub fn step(&mut self) {
        self.app
            .world
            .send_event::<SimulatorRequestEvent>(SimulatorRequestEvent::GetState);
        self.app.update();
    }

    pub fn spawn(
        &mut self,
        model: Model,
        position: Vec<f32>,
        color: Vec<f32>,
        rotation: Vec<f32>,
        scale: Vec<f32>,
    ) {
        self.app.world.send_event::<SpawnEvent>(SpawnEvent::Spawn {
            model,
            transform: Transform::from_xyz(position[0], position[1], position[2])
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    rotation[0],
                    rotation[1],
                    rotation[2],
                ))
                .with_scale(Vec3::new(scale[0], scale[1], scale[2])),
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

    pub fn publish_flatbuffers(&mut self, flatbuffer: Vec<u8>) {
        println!("Sending fb: {:?}", flatbuffer);
        self.app
            .world
            .send_event::<SimulatorRequestEvent>(SimulatorRequestEvent::PublishFlatBuffers(
                flatbuffer,
            ));
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

impl Default for KeskoApp {
    fn default() -> Self {
        Self {
            app: App::new(),
            log_level: Level::INFO,
        }
    }
}

fn start_scene(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            // Configure the projection to better fit the scene
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

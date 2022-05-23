use bevy::app::{App, Plugin};
use nora_physics::PhysicsPlugin;
use crate::cursor_tracking::{update_tracking_system, update_tracking_controller_system};


#[derive(Default)]
pub struct DefaultPhysicsPlugin;

impl Plugin for DefaultPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PhysicsPlugin::gravity())
            .add_system(update_tracking_system)
            .add_system(update_tracking_controller_system);
    }
}

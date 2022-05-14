pub(crate) mod core;
pub(crate) mod menu;
pub(crate) mod main_camera;
pub(crate) mod physics;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::math::Vec3;
use crate::plugins::{
    core::CorePlugin,
    menu::MenuPlugin,
    main_camera::MainCameraPlugin,
};
use nora_physics::PhysicsPlugin;


pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CorePlugin);
        group.add(MenuPlugin);
        group.add(MainCameraPlugin);
        group.add(PhysicsPlugin{gravity: Vec3::new(0.0, -9.81, 0.0)});
    }
}

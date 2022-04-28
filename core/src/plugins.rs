pub(crate) mod core;
pub(crate) mod menu;
pub(crate) mod main_camera;
pub(crate) mod physics;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use crate::plugins::{
    core::CorePlugin,
    menu::MenuPlugin,
    main_camera::MainCameraPlugin,
    physics::DefaultPhysicsPlugin
};


pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CorePlugin);
        group.add(MenuPlugin);
        group.add(MainCameraPlugin);
        group.add(DefaultPhysicsPlugin);
    }
}

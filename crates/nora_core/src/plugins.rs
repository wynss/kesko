pub(crate) mod core;
pub(crate) mod main_camera;
pub mod physics;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use nora_object_interaction::InteractionPlugin;
use crate::{
    interaction::groups::{GroupDynamic, GroupStatic},
    plugins::{
        core::CorePlugin,
        main_camera::MainCameraPlugin,
        physics::DefaultPhysicsPlugin
    },
    ui::UIPlugin
};


pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CorePlugin);
        group.add(UIPlugin);
        group.add(MainCameraPlugin);
        group.add(DefaultPhysicsPlugin);
        group.add(InteractionPlugin::<GroupDynamic>::default());
        group.add(InteractionPlugin::<GroupStatic>::default());
    }
}

pub(crate) mod core;
pub(crate) mod main_camera;
pub mod physics;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use kesko_object_interaction::InteractionPlugin;
use kesko_core::{
    interaction::groups::{GroupDynamic, GroupStatic},
};
use kesko_ui::UIPlugin;
use self::{
    core::CorePlugin,
    main_camera::MainCameraPlugin,
    physics::DefaultPhysicsPlugin
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
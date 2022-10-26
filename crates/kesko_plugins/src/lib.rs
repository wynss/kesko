pub mod main_camera;
pub mod physics;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub use kesko_object_interaction::InteractionPlugin;
use kesko_core::{
    interaction::groups::{GroupDynamic, GroupStatic},
};
pub use kesko_ui::UIPlugin;
use kesko_core::{CoreHeadlessPlugin, CorePlugin};
use self::{
    main_camera::MainCameraPlugin,
    physics::DefaultPhysicsPlugin
};
use kesko_models::ModelPlugin;


pub struct CorePlugins;
impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CorePlugin);
        group.add(UIPlugin);
        group.add(MainCameraPlugin);
        group.add(DefaultPhysicsPlugin);
        group.add(InteractionPlugin::<GroupDynamic>::default());
        group.add(InteractionPlugin::<GroupStatic>::default());
        group.add(ModelPlugin);
    }
}

/// Plugins for running in headless mode
pub struct HeadlessRenderPlugins;
impl PluginGroup for HeadlessRenderPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CoreHeadlessPlugin);
        group.add(DefaultPhysicsPlugin);
        group.add(ModelPlugin);
    }
}

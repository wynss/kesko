pub mod main_camera;
pub mod physics;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    utils::default,
};

use self::{main_camera::MainCameraPlugin, physics::DefaultPhysicsPlugin};
use kesko_core::interaction::groups::{GroupDynamic, GroupStatic};
use kesko_core::CorePlugin;
use kesko_models::ModelPlugin;
pub use kesko_object_interaction::InteractionPlugin;
pub use kesko_ui::UIPlugin;

pub struct CorePlugins;
impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin)
            .add(UIPlugin)
            .add(MainCameraPlugin)
            .add(DefaultPhysicsPlugin)
            .add(InteractionPlugin::<GroupDynamic>::default())
            .add(InteractionPlugin::<GroupStatic>::default())
            .add(ModelPlugin)
    }
}

/// Plugins for running in headless mode
pub struct HeadlessRenderPlugins {
    pub initial_physic_state: kesko_physics::PhysicState,
}
impl PluginGroup for HeadlessRenderPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin)
            .add(kesko_physics::PhysicsPlugin {
                initial_state: self.initial_physic_state,
                ..default()
            })
            .add(ModelPlugin)
    }
}

impl Default for HeadlessRenderPlugins {
    fn default() -> Self {
        Self {
            initial_physic_state: kesko_physics::PhysicState::Running,
        }
    }
}

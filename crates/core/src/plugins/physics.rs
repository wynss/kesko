use bevy::{
    app::{App, Plugin},
    math::Vec3
};
use heron::{PhysicsPlugin, Gravity};


#[derive(Default)]
pub(crate) struct DefaultPhysicsPlugin;

impl Plugin for DefaultPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin::default())
            .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)));
    }
}

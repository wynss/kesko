use bevy::ecs::component::Component;
use rapier3d::dynamics::RigidBodyHandle;

#[derive(Component)]
pub enum RigidBodyComp {
    Fixed,
    Dynamic
}

#[derive(Component)]
pub struct RigidBodyHandleComp(pub RigidBodyHandle);
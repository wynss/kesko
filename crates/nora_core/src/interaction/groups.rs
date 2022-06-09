/// This modules contains components that are used to group interactive entities.
/// 
/// This is implemented with generics so for example an entity with
/// InteractorBundle<GroupDynamic> bundle can only interact with an entity with
/// Interactive<GroupDynamic> bundle
use bevy::prelude::{Component};


/// For static entities
#[derive(Component, Default)]
pub struct GroupStatic;

/// For dynamic entities
#[derive(Component, Default)]
pub struct GroupDynamic;
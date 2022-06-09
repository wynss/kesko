pub mod debug;
pub mod event;
pub mod material;
mod interaction;

use std::marker::PhantomData;

use bevy::prelude::*;
use nora_raycast::{RayCastSource, RayCastSystems, RayCastable, RayCastPlugin};
use crate::{
    interaction::{
        DraggingGlobal, update_interactions,
        Drag, Hover
    },
    event::InteractionEvent,
    material::{InteractionMaterials, set_initial_interaction_material, OriginalMaterial}
};


#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
enum InteractionSystems {
    UpdateInteractions,
    SendEvents
}

#[derive(Default)]
pub struct InteractionPlugin<T>
where T: Component + Default 
{
    _phantom: PhantomData<fn() -> T>
}
impl<T> Plugin for InteractionPlugin<T> 
where T: Component + Default
{
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionMaterials>()
            .add_event::<InteractionEvent>()
            .init_resource::<DraggingGlobal>()
            .add_plugin(RayCastPlugin::<T>::default())
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_system(update_interactions::<T>
                        .label(InteractionSystems::UpdateInteractions)
                        .after(RayCastSystems::CalcIntersections)
                    )
                    .with_system(event::send_events
                        .label(InteractionSystems::SendEvents)
                        .after(InteractionSystems::UpdateInteractions))
                    .with_system(
                        debug::update_interaction_material.after(InteractionSystems::UpdateInteractions),
                    )
                    .with_system(set_initial_interaction_material),
            );
    }
}

#[derive(Bundle, Default)]
pub struct InteractiveBundle<T: Component + Default> {
    material: OriginalMaterial,
    ray_castable: RayCastable::<T>,
    drag: Drag,
    hover: Hover
}

#[derive(Bundle)]
pub struct InteractorBundle<T: Component + Default> {
    source: RayCastSource<T>,
}

impl<T> Default for InteractorBundle<T> 
where T: Component + Default {
    fn default() -> Self {
        Self {
            source: RayCastSource::<T>::screen_space(),
        }
    }
}

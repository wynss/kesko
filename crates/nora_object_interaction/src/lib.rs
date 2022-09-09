pub mod debug;
pub mod event;
pub mod material;
pub mod interaction;

use std::marker::PhantomData;

use bevy::prelude::*;
use event::SelectEvent;
use interaction::Select;
use nora_raycast::{RayCastSource, RayCastSystems, RayVisible, RayCastPlugin};
use crate::{
    interaction::{
        DraggingGlobal, update_interactions,
        Drag, Hover
    },
    event::InteractionEvent,
    material::{
        InteractionMaterials, 
        set_initial_interaction_material, 
        OriginalMaterial
    }
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
            .add_event::<SelectEvent>()
            .init_resource::<DraggingGlobal>()
            .add_plugin(RayCastPlugin::<T>::default())
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_system(update_interactions::<T>
                        .label(InteractionSystems::UpdateInteractions)
                        .after(RayCastSystems::CalcIntersections)
                    )
                    .with_system(event::handle_select_events::<T>)
                    .with_system(event::send_interaction_events::<T>
                        .label(InteractionSystems::SendEvents)
                        .after(InteractionSystems::UpdateInteractions))
                    .with_system(
                        debug::update_interaction_material::<T>.after(InteractionSystems::SendEvents),
                    )
                    .with_system(set_initial_interaction_material),
            );
    }
}

#[derive(Bundle, Default)]
pub struct InteractiveBundle<T: Component + Default> {
    material: OriginalMaterial,
    ray_castable: RayVisible::<T>,
    drag: Drag<T>,
    select: Select<T>,
    hover: Hover<T>
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

pub mod debug;
pub mod event;
pub mod interaction;
pub mod material;

use std::marker::PhantomData;

use crate::{
    event::InteractionEvent,
    interaction::{update_interactions, Drag, GlobalDragState, Hover},
    material::{set_initial_interaction_material, InteractionMaterials, OriginalMaterial},
};
use bevy::prelude::*;
use event::SelectEvent;
use interaction::Select;
use kesko_raycast::{RayCastPlugin, RayCastSource, RayVisible};

#[derive(Default)]
pub struct InteractionPlugin<T>
where
    T: Component + Default,
{
    _phantom: PhantomData<fn() -> T>,
}
impl<T> Plugin for InteractionPlugin<T>
where
    T: Component + Default,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionMaterials>()
            .add_event::<InteractionEvent>()
            .add_event::<SelectEvent>()
            .init_resource::<GlobalDragState>()
            .add_plugin(RayCastPlugin::<T>::default())
            .add_systems(
                Update,
                (
                    update_interactions::<T>,
                    event::send_interaction_events::<T>,
                    event::handle_select_events::<T>,
                    debug::update_interaction_material::<T>,
                    set_initial_interaction_material,
                )
                    .chain(),
            );
    }
}

#[derive(Bundle, Default)]
pub struct InteractiveBundle<T: Component + Default> {
    material: OriginalMaterial,
    ray_castable: RayVisible<T>,
    drag: Drag<T>,
    select: Select<T>,
    hover: Hover<T>,
}

#[derive(Bundle)]
pub struct InteractorBundle<T: Component + Default> {
    source: RayCastSource<T>,
}

impl<T> Default for InteractorBundle<T>
where
    T: Component + Default,
{
    fn default() -> Self {
        Self {
            source: RayCastSource::<T>::screen_space(),
        }
    }
}

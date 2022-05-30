pub mod debug;
pub mod event;
pub mod material;
mod interaction;

use bevy::prelude::*;
use nora_raycast::{RayCastPlugin, RayCastSource, RayCastSystems, RayCastable};
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

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionMaterials>()
            .add_event::<InteractionEvent>()
            .init_resource::<DraggingGlobal>()
            .add_plugin(RayCastPlugin::default())
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_system(update_interactions
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
pub struct InteractiveBundle {
    material: OriginalMaterial,
    ray_castable: RayCastable,
    drag: Drag,
    hover: Hover
}

#[derive(Bundle)]
pub struct InteractorBundle {
    source: RayCastSource,
}

impl Default for InteractorBundle {
    fn default() -> Self {
        Self {
            source: RayCastSource::screen_space(),
        }
    }
}

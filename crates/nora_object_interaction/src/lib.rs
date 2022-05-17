pub mod debug;
pub mod event;
mod interaction;

use bevy::prelude::*;
use nora_raycast::{RayCastPlugin, RayCastSource, RayCastSystems, RayCastable};
use crate::event::InteractionEvent;
use crate::interaction::{
    DraggingGlobal, update_interactions,
    Drag, Hover
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
            .add_plugin(RayCastPlugin)
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

#[derive(Component, Default)]
struct OriginalMaterial(Option<Handle<StandardMaterial>>);

struct InteractionMaterials {
    hovered: Handle<StandardMaterial>,
    pressed: Handle<StandardMaterial>,
}

impl FromWorld for InteractionMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        Self {
            hovered: materials.add(Color::GOLD.into()),
            pressed: materials.add(Color::INDIGO.into()),
        }
    }
}

fn set_initial_interaction_material(
    mut query: Query<(&mut OriginalMaterial, &Handle<StandardMaterial>)>,
) {
    for (mut original_material, material) in query.iter_mut() {
        if original_material.0.is_none() {
            original_material.0 = Some(material.clone());
        }
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

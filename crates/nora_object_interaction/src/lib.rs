mod debug;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use nora_raycast::{RayCastPlugin, RayCastSource, RayCastSystems, RayCastable};


#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
enum InteractionSystems {
    UpdateInteractions,

}

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionMaterials>()
            .add_event::<InteractionEvent>()
            .init_resource::<Dragging>()
            .add_plugin(RayCastPlugin)
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_system(update_interactions
                        .label(InteractionSystems::UpdateInteractions)
                        .after(RayCastSystems::CalcIntersections)
                    )
                    .with_system(
                        debug::update_interaction_material.after(InteractionSystems::UpdateInteractions),
                    )
                    .with_system(set_initial_interaction_material),
            );
    }
}

#[derive(Component, PartialEq)]
pub(crate) enum InteractionState {
    Pressed,
    Hovered,
    None,
}

#[derive(Default)]
struct Dragging {
    is_dragging: bool
}

#[derive(Component, Default)]
struct OriginalMaterial(Option<Handle<StandardMaterial>>);

struct InteractionMaterials {
    selected: Handle<StandardMaterial>,
    hovered: Handle<StandardMaterial>,
    pressed: Handle<StandardMaterial>,
}

impl FromWorld for InteractionMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        Self {
            selected: materials.add(Color::GOLD.into()),
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

pub enum InteractionEvent {
    Pressed(Entity),
    Selected(Entity),
    Hovered(Entity),
    None(Entity),
}

fn update_interactions(
    mut motion_evr: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    source_query: Query<&RayCastSource, With<Camera>>,
    mut dragging: Local<Dragging>,
    mut interaction_query: Query<(Entity, &mut InteractionState)>,
) {
    let mouse_pressed = mouse_button_input.pressed(MouseButton::Left);
    let mouse_just_released = mouse_button_input.just_released(MouseButton::Left);

    // get if the cursor moved
    let mut mouse_motion = 0.0;
    for motion in motion_evr.iter() {
        mouse_motion += motion.delta.x.abs() + motion.delta.y.abs();
    }
    let cursor_moved = mouse_motion > 0.5;

    if let Ok(source) = source_query.get_single() {

        let hit_entity = source.ray_hit.as_ref().map(|hit| hit.entity);

        if mouse_pressed && cursor_moved && !dragging.is_dragging{
            if let Some(hit) = &source.ray_hit {
                let (_, mut i) = interaction_query.get_mut(hit.entity).unwrap();
                if *i != InteractionState::Pressed {
                    *i = InteractionState::Pressed;
                    dragging.is_dragging = true;
                }
            }
        }
        else if mouse_just_released {
            for (e, mut i) in interaction_query.iter_mut() {
                if hit_entity == Some(e) && *i == InteractionState::Pressed {
                    *i = InteractionState::Hovered;
                } else if hit_entity != Some(e) && *i != InteractionState::None{
                    *i = InteractionState::None;
                }
                dragging.is_dragging = false;
            }
        } else {
            for (e, mut i) in interaction_query.iter_mut() {
                if hit_entity == Some(e) && *i != InteractionState::Hovered && *i != InteractionState::Pressed {
                    *i = InteractionState::Hovered;
                } else if hit_entity != Some(e) && *i == InteractionState::Hovered {
                    *i = InteractionState::None;
                }
            }
        }
    }
}

#[derive(Bundle)]
pub struct InteractiveBundle {
    material: OriginalMaterial,
    ray_castable: RayCastable,
    interaction: InteractionState,
}

impl Default for InteractiveBundle {
    fn default() -> Self {
        Self {
            material: OriginalMaterial::default(),
            ray_castable: RayCastable::default(),
            interaction: InteractionState::None,
        }
    }
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

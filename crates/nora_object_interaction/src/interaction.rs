use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use nora_raycast::RayCastSource;


#[derive(Default)]
pub(crate) struct Dragging {
    is_dragging: bool
}

#[derive(Component, PartialEq)]
pub(crate) enum InteractionState {
    Dragged,
    Hovered,
    None,
}

pub(crate) fn update_interactions(
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
            if let Some(hit_entity) = hit_entity {
                let (_, mut i) = interaction_query.get_mut(hit_entity).unwrap();
                if *i != InteractionState::Dragged {
                    *i = InteractionState::Dragged;
                    dragging.is_dragging = true;
                }
            }
        }
        else if mouse_just_released {
            for (e, mut i) in interaction_query.iter_mut() {
                if hit_entity == Some(e) && *i == InteractionState::Dragged {
                    *i = InteractionState::Hovered;
                } else if hit_entity != Some(e) && *i != InteractionState::None{
                    *i = InteractionState::None;
                }
                dragging.is_dragging = false;
            }
        } else {
            for (e, mut i) in interaction_query.iter_mut() {
                if hit_entity == Some(e) && *i != InteractionState::Hovered && *i != InteractionState::Dragged {
                    *i = InteractionState::Hovered;
                } else if hit_entity != Some(e) && *i == InteractionState::Hovered {
                    *i = InteractionState::None;
                }
            }
        }
    }
}

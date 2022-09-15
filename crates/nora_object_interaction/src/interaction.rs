use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use nora_raycast::{RayCastSource, RayCastMethod};

const CURSOR_MOVE_LIMIT: f32 = 0.5;


#[derive(Default)]
pub(crate) struct GlobalDragState {
    dragged: bool,
    block_drag: bool
}

#[derive(Component, Default)]
pub struct Drag<T: Component + Default> {
    pub dragged: bool,
    _phantom: PhantomData<fn() -> T>
}

#[derive(Component, Default)]
pub struct Hover<T: Component + Default> {
    pub hovered: bool,
    _phantom: PhantomData<fn() -> T>
}

#[derive(Component, Default)]
pub struct Select<T: Component + Default> {
    pub selected: bool,
    _phantom: PhantomData<fn() -> T>
}

/// System for updating the user interactions with objects
#[allow(clippy::type_complexity)]
pub(crate) fn update_interactions<T: Component + Default>(
    mut motion_evr: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    source_query: Query<&RayCastSource<T>, With<Camera>>,
    mut global_drag: ResMut<GlobalDragState>,
    mut interaction_query: Query<(Entity, &mut Drag<T>, &mut Hover<T>, &mut Select<T>)>,
) {

    let left_btn_pressed = mouse_button_input.pressed(MouseButton::Left);
    let left_btn_just_released = mouse_button_input.just_released(MouseButton::Left);
    let left_btn_just_pressed = mouse_button_input.just_pressed(MouseButton::Left);

    // get if the cursor moved
    let mouse_motion: f32 = motion_evr.iter().map(|m| m.delta.length()).sum();
    let cursor_moved = mouse_motion > CURSOR_MOVE_LIMIT;

    if let Ok(source) = source_query.get_single() {
        // We have a single source that casts rays

        // Make sure the source is casting from screen space
        if let RayCastMethod::ScreenSpace = source.method {

            // possible entity that was hit by the ray
            let entity_hit = source.ray_hit.as_ref().map(|hit| hit.entity);
            
            // block dragging if we are pressing left mouse and are not hovering over something.
            // otherwise an object will be dragged as soon as the cursor is over it
            if entity_hit.is_none() && left_btn_just_pressed {
                global_drag.block_drag = true;
            } else if left_btn_just_released && global_drag.block_drag {
                global_drag.block_drag = false;
            }

            // loop over object and update the state of their interactive components
            for (entity, mut drag, mut hover, mut select) in interaction_query.iter_mut() {

                if entity_hit == Some(entity) {
                    // we are pointing at 'entity'

                    // handle hover
                    if !hover.hovered {
                        hover.hovered = true;
                    }
                    
                    // handle selection
                    if left_btn_just_released && !cursor_moved && !global_drag.dragged{
                        select.selected = !select.selected;
                    }

                    // handle drag
                    if left_btn_pressed && cursor_moved && !drag.dragged && !global_drag.dragged && !global_drag.block_drag {
                        drag.dragged = true;
                        global_drag.dragged = true;
                    } else if left_btn_just_released && drag.dragged {
                        drag.dragged = false;
                        global_drag.dragged = false;
                    }
                    
                } else {
                    // we are not pointing at 'entity'
                    
                    //handle hover
                    if hover.hovered {
                        hover.hovered = false;
                    }

                    // handle drag
                    if left_btn_just_released && drag.dragged {
                        drag.dragged = false;
                        global_drag.dragged = false
                    }
                }
            } 
        }
    }
}

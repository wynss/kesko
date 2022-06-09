use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use nora_raycast::RayCastSource;


#[derive(Default)]
pub(crate) struct DraggingGlobal {
    dragged: bool
}

#[derive(Component, Default)]
pub(crate) struct Drag<T: Component + Default> {
    pub(crate) dragged: bool,
    _phantom: PhantomData<fn() -> T>
}

#[derive(Component, Default)]
pub(crate) struct Hover<T: Component + Default> {
    pub(crate) hovered: bool,
    _phantom: PhantomData<fn() -> T>
}

pub(crate) fn update_interactions<T: Component + Default>(
    mut motion_evr: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    source_query: Query<&RayCastSource<T>, With<Camera>>,
    mut dragging_global: ResMut<DraggingGlobal>,
    mut interaction_query: Query<(Entity, &mut Drag<T>, &mut Hover<T>)>,
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

        if mouse_pressed && cursor_moved && !dragging_global.dragged {
            if let Some(hit_entity) = hit_entity {
                let (_, mut drag, _) = interaction_query.get_mut(hit_entity).unwrap();
                if !drag.dragged {
                    drag.dragged = true;
                    dragging_global.dragged = true;
                }
            }
        }
        else if mouse_just_released {
            for (e, mut drag, mut hover) in interaction_query.iter_mut() {
                if hit_entity == Some(e) && drag.dragged {
                    hover.hovered = true;
                    drag.dragged = false;
                } else if hit_entity != Some(e) {
                    if drag.dragged {
                        drag.dragged = false;
                    }
                    if hover.hovered {
                        hover.hovered = false;
                    }
                }
                dragging_global.dragged = false;
            }
        } else {
            for (e, drag, mut hover) in interaction_query.iter_mut() {
                if hit_entity == Some(e) && !drag.dragged && !hover.hovered {
                    hover.hovered = true;
                } else if hit_entity != Some(e) && hover.hovered {
                    hover.hovered = false;
                }
            }
        }
    }
}

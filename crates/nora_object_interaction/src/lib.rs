use bevy::prelude::*;
use bevy::render::render_graph::NodeRunError::InputSlotError;
use nora_raycast::{RayCastPlugin, RayCastSource, RayCastSystems, RayCastable};


#[derive(Component, PartialEq)]
pub(crate) enum Interaction {
    Pressed,
    Hovered,
    Selected,
    None
}


pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionMaterials>()
            .add_plugin(RayCastPlugin)
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_system(update_interaction_material.after(RayCastSystems::CalcIntersections))
                    .with_system(update_interactions)
                    .with_system(set_initial_interaction_material)
            );
    }
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
        let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
        Self {
            selected: materials.add(Color::GOLD.into()),
            hovered: materials.add(Color::INDIGO.into()),
            pressed: materials.add(Color::AQUAMARINE.into()),
        }
    }
}

fn set_initial_interaction_material(
    mut query: Query<(&mut OriginalMaterial, &Handle<StandardMaterial>)>
) {
    for (mut original_material, material) in query.iter_mut() {

        if original_material.0.is_none() {
            original_material.0 = Some(material.clone());
        }
    }
}


fn update_interactions(
    source_query: Query<&RayCastSource>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut interaction_query: Query<(Entity, &mut Interaction)>
) {

    if mouse_button_input.just_pressed(MouseButton::Left) {
        for source in source_query.iter() {
            if let Some(hit) = &source.ray_hit {
                if let Ok((_, mut interaction)) = interaction_query.get_mut(hit.entity) {
                    if *interaction == Interaction::None {
                        *interaction = Interaction::Pressed;
                    }
                }
            }
        }
    } else if mouse_button_input.just_released(MouseButton::Left) {
        for source in source_query.iter() {
            if let Some(hit) = &source.ray_hit {
                for (entity, mut interaction) in interaction_query.iter_mut() {
                    if entity == hit.entity {
                        if *interaction == Interaction::Pressed {
                            *interaction = Interaction::Selected;
                        } else if *interaction == Interaction::Selected {
                            *interaction = Interaction::None;
                        }
                    } else if *interaction == Interaction::Pressed {
                        *interaction = Interaction::None;
                    }
                }
            } else {
                for (_, mut interaction) in interaction_query.iter_mut() {
                    if *interaction == Interaction::Pressed {
                        *interaction = Interaction::None;
                    }
                }
            }
        }
    } else {
        for source in source_query.iter() {
            if let Some(hit) = &source.ray_hit {
                for (entity, mut interaction) in interaction_query.iter_mut() {
                    if *interaction == Interaction::None && entity == hit.entity {
                        *interaction = Interaction::Hovered;
                    } else if *interaction == Interaction::Hovered && entity != hit.entity {
                        *interaction = Interaction::None
                    }
                }
            } else {
                for (_, mut interaction) in interaction_query.iter_mut() {
                    if *interaction == Interaction::Hovered {
                        *interaction = Interaction::None
                    }
                }

            }
        }
    }
}

fn update_mouse_interaction(mut interaction: Mut<Interaction>, mouse_button_input: &Input<MouseButton>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        *interaction = Interaction::Pressed;
    } else if mouse_button_input.just_released(MouseButton::Left) {
        *interaction = Interaction::Selected;
    } else {
        *interaction = Interaction::Hovered;
    }
}

fn update_interaction_material(
    interaction_materials: Res<InteractionMaterials>,
    mut interaction_query: Query<(&Interaction, &OriginalMaterial, &mut Handle<StandardMaterial>), Changed<Interaction>>
) {

    for (interaction, original_material, mut current_material) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => *current_material = interaction_materials.pressed.clone(),
            Interaction::Selected => *current_material = interaction_materials.selected.clone(),
            Interaction::Hovered => *current_material = interaction_materials.hovered.clone(),
            Interaction::None => {
                if let Some(material) = &original_material.0 {
                    *current_material = material.clone()
                }
            }
        }
    }
}

#[derive(Bundle)]
pub struct InteractiveBundle {
    material: OriginalMaterial,
    ray_castable: RayCastable,
    interaction: Interaction
}

impl Default for InteractiveBundle {
    fn default() -> Self {
        Self {
            material: OriginalMaterial::default(),
            ray_castable: RayCastable::default(),
            interaction: Interaction::None
        }
    }
}

#[derive(Bundle)]
pub struct InteractorBundle {
    source: RayCastSource
}

impl Default for InteractorBundle {
    fn default() -> Self {
        Self {
            source: RayCastSource::screen_space()
        }
    }
}

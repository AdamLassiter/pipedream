use crate::cards::event::{NodeInteraction, NodeInteractionType};
use crate::cards::{DropZoneNode, HoldingState, InteractiveNode, LerpTarget};
use bevy::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DRAG_DURATION: Duration = Duration::from_millis(60);
const LERP_STRENGTH: f32 = 0.999;

pub fn get_timestamp() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_millis()
}

pub fn lerp_drop_zone(cursor: Vec2, drop_zones: &Vec<&DropZoneNode>) -> Option<LerpTarget> {
    let mut intersects = drop_zones
        .iter()
        .filter(|&drop_zone| drop_zone.contains(cursor))
        .collect::<Vec<_>>();
    intersects.sort_by(|&x, &y| x.distance(cursor).total_cmp(&y.distance(cursor)));
    intersects.first().map(|&drop_zone| LerpTarget {
        position: drop_zone.position,
        strength: LERP_STRENGTH,
    })
}

pub fn handle_hover(
    node_interaction_events: &mut EventWriter<'_, NodeInteraction>,
    is_hover: bool,
    active: Entity,
) {
    if is_hover {
        node_interaction_events.write(NodeInteraction {
            entity: active,
            interaction_type: NodeInteractionType::Hover,
        });
    }
}

pub fn handle_pickup_entity(
    buttons: &Res<'_, ButtonInput<MouseButton>>,
    node_interaction_events: &mut EventWriter<'_, NodeInteraction>,
    holding_state: &mut Local<'_, HoldingState>,
    now_ms: u128,
    is_hover: &mut bool,
    active: Entity,
) {
    // Left and right click
    if buttons.just_pressed(MouseButton::Left) {
        *is_hover = false;
        **holding_state = HoldingState {
            duration: Duration::from_millis(now_ms as u64),
            entity: Some(active),
            is_holding: false,
        };
    }
    if buttons.just_pressed(MouseButton::Right) {
        *is_hover = false;
        node_interaction_events.write(NodeInteraction {
            entity: active,
            interaction_type: NodeInteractionType::RightClick,
        });
    }
}

pub fn handle_hold_entity(
    buttons: &Res<'_, ButtonInput<MouseButton>>,
    node_interaction_events: &mut EventWriter<'_, NodeInteraction>,
    holding_state: &mut Local<'_, HoldingState>,
    now_ms: u128,
    is_hover: &mut bool,
    active: Entity,
) {
    // Left hold
    if buttons.pressed(MouseButton::Left)
        && Duration::from_millis(now_ms as u64) - holding_state.duration > DRAG_DURATION
    {
        *is_hover = false;
        holding_state.is_holding = true;
        let held_entity = *holding_state.entity.get_or_insert(active);
        node_interaction_events.write(NodeInteraction {
            entity: held_entity,
            interaction_type: NodeInteractionType::LeftDrag,
        });
    }
}

pub fn handle_drop_entity(
    buttons: &Res<'_, ButtonInput<MouseButton>>,
    node_interaction_events: &mut EventWriter<'_, NodeInteraction>,
    holding_state: &mut Local<'_, HoldingState>,
    active: Entity,
) {
    // Left drop
    if buttons.just_released(MouseButton::Left) {
        if let Some(dropped_entity) = holding_state.entity
            && holding_state.is_holding
        {
            node_interaction_events.write(NodeInteraction {
                entity: dropped_entity,
                interaction_type: NodeInteractionType::LeftDrop,
            });
        } else {
            node_interaction_events.write(NodeInteraction {
                entity: active,
                interaction_type: NodeInteractionType::LeftClick,
            });
        }
        **holding_state = HoldingState {
            is_holding: false,
            duration: Duration::ZERO,
            entity: None,
        };
    }
}

pub fn handle_drop_deleted_entity(
    mut node_interaction_events: EventWriter<'_, NodeInteraction>,
    mut holding_state: Local<'_, HoldingState>,
) {
    // If held entity is deleted
    *holding_state = HoldingState {
        is_holding: false,
        duration: Duration::ZERO,
        entity: None,
    };
    node_interaction_events.write(NodeInteraction {
        entity: Entity::PLACEHOLDER,
        interaction_type: NodeInteractionType::LeftDrop,
    });
}

pub fn get_active_entity(
    cursor_moved_events: EventReader<'_, '_, CursorMoved>,
    buttons: &Res<'_, ButtonInput<MouseButton>>,
    res_images: Res<'_, Assets<Image>>,
    sprite_query: Query<'_, '_, (&Sprite, &GlobalTransform, Entity), With<InteractiveNode>>,
    primary_window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    scale_factor: f32,
) -> Option<(Entity, f32)> {
    let mut active_entity = None;
    if !cursor_moved_events.is_empty() || buttons.get_pressed().size_hint().0 > 0 {
        for (sprite, node_transform, entity) in sprite_query.iter() {
            let size = match sprite.custom_size {
                Some(size) => (size.x, size.y),
                None => {
                    if let Some(sprite_image) = res_images.get(&sprite.image) {
                        (
                            sprite_image.size().x as f32 / scale_factor,
                            sprite_image.size().y as f32 / scale_factor,
                        )
                    } else {
                        (1., 1.)
                    }
                }
            };

            // Find the bounds for this sprite
            let x_min = node_transform.affine().translation.x - size.0 / 2.;
            let y_min = node_transform.affine().translation.y - size.1 / 2.;
            let x_max = node_transform.affine().translation.x + size.0 / 2.;
            let y_max = node_transform.affine().translation.y + size.1 / 2.;
            let z_current = node_transform.affine().translation.z;

            if let Some(pos) = primary_window.cursor_position()
                && let Ok(pos) = camera.viewport_to_world_2d(camera_transform, pos)
                && x_min < pos.x
                && pos.x < x_max
                && y_min < pos.y
                && pos.y < y_max
            {
                if let Some((_, z)) = active_entity {
                    // Prefer the largest z-index if covering two sprites
                    if z < z_current {
                        active_entity = Some((entity, z_current));
                    }
                } else {
                    // Pick this if just one sprite
                    active_entity = Some((entity, node_transform.affine().translation.z));
                }
            }
        }
    }
    active_entity
}

use bevy::{prelude::*, window::PrimaryWindow};

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::MainCamera;

use super::event::{NodeInteraction, NodeInteractionType};

const DRAG_DURATION: Duration = Duration::from_millis(60);

#[derive(Default)]
pub struct LerpTarget {
    position: Vec2,
    strength: f32,
}

impl LerpTarget {
    fn lerp(&self, translation: Vec3, delta: Duration) -> Vec3 {
        let s = self.strength * delta.as_secs_f32() * 10.;
        Vec3::new(
            translation.x * (1. - s) + self.position.x * s,
            translation.y * (1. - s) + self.position.y * s,
            translation.z,
        )
    }
}

#[derive(Component, Default)]
pub struct InteractiveNode {
    target: LerpTarget,
}

#[derive(Default)]
struct HoldingState {
    duration: Duration,
    entity: Option<Entity>,
    is_holding: bool,
}

fn get_timestamp() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_millis()
}

fn follow_drag_event(
    mut node_interaction_events: EventReader<NodeInteraction>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut sprite_query: Query<(&Sprite, &mut InteractiveNode, Entity)>,
) {
    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera.single().unwrap();
    let interactions = node_interaction_events.read().collect::<Vec<_>>();
    for (_sprite, mut inode, entity) in sprite_query.iter_mut() {
        if let Some(&interaction) = interactions
            .iter()
            .find(|&interaction| interaction.entity == entity)
        {
            match interaction.interaction_type {
                NodeInteractionType::LeftDrag => {
                    if let Some(cursor_transform) = window.cursor_position().and_then(|cursor| {
                        camera.viewport_to_world_2d(camera_transform, cursor).ok()
                    }) {
                        inode.target = LerpTarget {
                            position: cursor_transform,
                            strength: 0.5,
                        }
                    }
                }
                _ => { /* do nothing */ }
            }
        }
    }
}

fn lerp_to_target(
    mut sprite_query: Query<(&Sprite, &mut Transform, &InteractiveNode)>,
    time: Res<Time>,
) {
    for (_sprite, mut transform, inode) in sprite_query.iter_mut() {
        transform.translation = inode.target.lerp(transform.translation, time.delta());
    }
}

fn interactive_sprite(
    cursor_moved_events: EventReader<CursorMoved>,
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    res_images: Res<Assets<Image>>,
    sprite_query: Query<(&Sprite, &GlobalTransform, Entity), With<InteractiveNode>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut node_interaction_events: EventWriter<NodeInteraction>,
    mut holding_state: Local<HoldingState>,
) {
    for (camera, camera_transform) in camera_query.iter() {
        for primary_window in windows.iter() {
            let scale_factor = primary_window.scale_factor();
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
                            if z < z_current {
                                active_entity = Some((entity, z_current));
                            }
                        } else {
                            active_entity = Some((entity, node_transform.affine().translation.z));
                        }
                    }
                }
            }

            if let Some(active) = active_entity
                .map(|active| active.0)
                .or(holding_state.entity)
            {
                let now_ms = get_timestamp();
                let mut is_hover = true;
                if buttons.just_pressed(MouseButton::Left) {
                    is_hover = false;
                    *holding_state = HoldingState {
                        duration: Duration::from_millis(now_ms as u64),
                        entity: Some(active),
                        is_holding: false,
                    };
                }
                if buttons.just_pressed(MouseButton::Right) {
                    is_hover = false;
                    node_interaction_events.write(NodeInteraction {
                        entity: active,
                        interaction_type: NodeInteractionType::RightClick,
                    });
                }

                if buttons.pressed(MouseButton::Left)
                    && Duration::from_millis(now_ms as u64) - holding_state.duration > DRAG_DURATION
                    && holding_state.entity.is_some()
                {
                    is_hover = false;
                    holding_state.is_holding = true;
                    node_interaction_events.write(NodeInteraction {
                        entity: active,
                        interaction_type: NodeInteractionType::LeftDrag,
                    });
                }

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
                    *holding_state = HoldingState {
                        is_holding: false,
                        duration: Duration::ZERO,
                        entity: None,
                    };
                }

                if is_hover {
                    node_interaction_events.write(NodeInteraction {
                        entity: active,
                        interaction_type: NodeInteractionType::Hover,
                    });
                }
            } else if buttons.just_released(MouseButton::Left) {
                // if held entity is deleted
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
        }
    }
}

#[derive(Default)]
pub struct InteractiveSpritesPlugin;

impl Plugin for InteractiveSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, interactive_sprite)
            .add_systems(Update, (follow_drag_event, lerp_to_target));
    }
}

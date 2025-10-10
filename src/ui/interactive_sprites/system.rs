use super::{DropZoneNode, HoldingState, InteractiveNode, LerpTarget, utils::*};
use crate::MainCamera;
use crate::ui::event::{NodeInteraction, NodeInteractionType};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn follow_drag_event(
    mut node_interaction_events: EventReader<NodeInteraction>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut sprite_query: Query<(&Sprite, &mut InteractiveNode, Entity)>,
    drop_zone_query: Query<&DropZoneNode>,
) {
    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera.single().unwrap();
    let interactions = node_interaction_events.read().collect::<Vec<_>>();
    let drop_zones = drop_zone_query.iter().collect::<Vec<_>>();
    for (_sprite, mut interactive_node, entity) in sprite_query.iter_mut() {
        if let Some(&interaction) = interactions
            .iter()
            .find(|&interaction| interaction.entity == entity)
        {
            match interaction.interaction_type {
                NodeInteractionType::LeftDrag => {
                    if let Some(cursor_transform) = window.cursor_position().and_then(|cursor| {
                        camera.viewport_to_world_2d(camera_transform, cursor).ok()
                    }) {
                        interactive_node.next_drop = lerp_drop_zone(cursor_transform, &drop_zones);
                        interactive_node.lerp_target =
                            interactive_node.next_drop.clone().unwrap_or(LerpTarget {
                                position: cursor_transform,
                                strength: 0.5,
                            });
                    }
                }
                _ => { /* do nothing */ }
            }
        }
    }
}

pub fn update_last_drop(
    mut node_interaction_events: EventReader<NodeInteraction>,
    mut sprite_query: Query<(&mut InteractiveNode, Entity)>,
) {
    for interaction_event in node_interaction_events.read() {
        if let NodeInteractionType::LeftDrop = interaction_event.interaction_type
            && let Ok((mut interactive_node, _)) = sprite_query.get_mut(interaction_event.entity)
        {
            interactive_node.last_drop = interactive_node.next_drop.clone();
        }
    }
}

pub fn lerp_to_target(
    mut sprite_query: Query<(&Sprite, &mut Transform, &InteractiveNode)>,
    time: Res<Time>,
) {
    for (_sprite, mut transform, inode) in sprite_query.iter_mut() {
        transform.translation = inode.lerp_target.lerp(transform.translation, time.delta());
    }
}

pub fn drag_drop_sprite(
    cursor_moved_events: EventReader<CursorMoved>,
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    res_images: Res<Assets<Image>>,
    sprite_query: Query<(&Sprite, &GlobalTransform, Entity)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut node_interaction_events: EventWriter<NodeInteraction>,
    mut holding_state: Local<HoldingState>,
) {
    let primary_window = windows.single().unwrap();
    let (camera, camera_transform) = camera_query.single().unwrap();
    let scale_factor = primary_window.scale_factor();

    // Get the entity under mouse hover
    let active_entity = get_active_entity(
        cursor_moved_events,
        &buttons,
        res_images,
        sprite_query,
        primary_window,
        camera,
        camera_transform,
        scale_factor,
    );

    // If there was a hovered entity, or last frame there was a held entity
    if let Some(active) = active_entity
        .map(|active| active.0)
        .or(holding_state.entity)
    {
        let now_ms = get_timestamp();
        let mut is_hover = true;

        handle_pickup_entity(
            &buttons,
            &mut node_interaction_events,
            &mut holding_state,
            now_ms,
            &mut is_hover,
            active,
        );
        handle_hold_entity(
            &buttons,
            &mut node_interaction_events,
            &mut holding_state,
            now_ms,
            &mut is_hover,
            active,
        );
        handle_drop_entity(
            &buttons,
            &mut node_interaction_events,
            &mut holding_state,
            active,
        );
        handle_hover(&mut node_interaction_events, is_hover, active);
    } else if buttons.just_released(MouseButton::Left) {
        handle_drop_deleted_entity(node_interaction_events, holding_state);
    }
}

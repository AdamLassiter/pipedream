mod system;
mod utils;

use bevy::prelude::*;
use std::time::Duration;

#[derive(Default, Clone)]
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
pub struct DropZoneNode {
    position: Vec2,
    size: Vec2,
}

impl DropZoneNode {
    fn as_rect(&self) -> Rect {
        Rect::from_center_size(self.position, self.size)
    }

    fn contains(&self, point: Vec2) -> bool {
        self.as_rect().contains(point)
    }

    fn distance(&self, point: Vec2) -> f32 {
        self.position.distance(point)
    }
}

pub struct DropZoneBundle {
    drop_zone: DropZoneNode,
    debug_rect: Rectangle,
}
impl DropZoneBundle {
    fn new(drop_zone: DropZoneNode) -> Self {
        let debug_rect = Rectangle::new(drop_zone.size.x, drop_zone.size.y);
        Self {
            drop_zone,
            debug_rect,
        }
    }
}

#[derive(Component, Default)]
pub struct InteractiveNode {
    lerp_target: LerpTarget,
    last_drop: Option<LerpTarget>,
    next_drop: Option<LerpTarget>,
}

#[derive(Default)]
struct HoldingState {
    duration: Duration,
    entity: Option<Entity>,
    is_holding: bool,
}

#[derive(Default)]
pub struct InteractiveSpritesPlugin;

impl Plugin for InteractiveSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, system::drag_drop_sprite)
            .add_systems(
                Update,
                (
                    system::follow_drag_event,
                    system::lerp_to_target,
                    system::update_last_drop,
                ),
            );
    }
}

mod event;
mod script;
mod system;
mod utils;

use bevy::prelude::*;
use std::time::Duration;

use crate::cards::{event::EventsPlugin, script::ScriptsPlugin, system::SystemsPlugin};

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

#[derive(Component, Default, Debug)]
pub struct DropZoneNode {
    position: Vec2,
    size: Vec2,
    debug_rect: Rectangle,
}

impl DropZoneNode {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        let debug_rect = Rectangle::new(size.x, size.y);
        Self {
            position,
            size,
            debug_rect,
        }
    }

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

#[derive(Component, Default)]
pub struct InteractiveNode {
    lerp_target: LerpTarget,
    last_drop: Option<LerpTarget>,
    next_drop: Option<LerpTarget>,
}

#[derive(Component)]
pub struct CardBacking {
    image: String,
}

impl Default for CardBacking {
    fn default() -> Self {
        Self { image: "cards/front.png".to_string() }
    }
}

#[derive(Default)]
struct HoldingState {
    duration: Duration,
    entity: Option<Entity>,
    is_holding: bool,
}

#[derive(Default)]
pub struct InteractiveCardsPlugin;

impl Plugin for InteractiveCardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EventsPlugin, ScriptsPlugin, SystemsPlugin));
    }
}

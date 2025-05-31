use bevy::prelude::*;

use crate::battle::stats::Stat;

#[derive(Event)]
pub struct EndTurnEvent;

#[derive(Event)]
pub struct StatChangeEvent {
    pub source: Entity,
    pub target: Entity,
    pub stat: Stat,
    pub change: i64,
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndTurnEvent>();
    }
}

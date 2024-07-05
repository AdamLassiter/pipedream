use std::collections::BTreeMap;

use serde::Serialize;

use crate::engine::{
    combat::{card::Cards, npc::Npcs},
    core::{
        location::Location,
        state::State,
        tag::{Static, TagKey},
    },
    state::combat_state_machine::CombatStateMachine,
};

pub static PLAYER: Static<TagKey> = Static::new(|| "player".into());
pub static ENEMY: Static<TagKey> = Static::new(|| "enemy".into());

pub static PLAYER_HAND: Static<TagKey> = Static::new(|| "player:hand".into());
pub static PLAYER_DECK: Static<TagKey> = Static::new(|| "player:deck".into());
pub static ENEMY_HAND: Static<TagKey> = Static::new(|| "enemy:hand".into());
pub static ENEMY_DECK: Static<TagKey> = Static::new(|| "enemy:deck".into());

pub static MY_HAND: Static<TagKey> = Static::new(|| "$my:hand".into());
pub static MY_DECK: Static<TagKey> = Static::new(|| "$my:deck".into());
pub static MY_DRAW_COUNT: Static<TagKey> = Static::new(|| "$my:draw-count".into());
pub static YOUR_HAND: Static<TagKey> = Static::new(|| "$your:hand".into());
pub static YOUR_DECK: Static<TagKey> = Static::new(|| "$your:deck".into());
pub static YOUR_DRAW_COUNT: Static<TagKey> = Static::new(|| "$your:draw-count".into());

type StateFn = dyn Fn(&CombatStateMachine) -> State + Send + Sync;

pub struct DynamicStateFn {
    pub func: Box<StateFn>,
}

impl DynamicStateFn {
    pub fn apply(&self, machine: &CombatStateMachine) -> State {
        (self.func)(machine)
    }
}

impl DynamicStateFn {
    pub fn new(func: fn(&CombatStateMachine) -> State) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

#[derive(Serialize)]
pub struct CombatWorld {
    #[serde(skip_serializing)]
    pub states: BTreeMap<Location, DynamicStateFn>,
    pub cards: Cards,
    pub npcs: Npcs,
}

impl CombatWorld {
    pub fn get_state(&self, location: &Location) -> &DynamicStateFn {
        self.states
            .get(location)
            .unwrap_or_else(|| panic!("Failed to find location {:?} in combat world", location.0))
    }
}

use std::collections::BTreeMap;

use serde::Serialize;

use crate::engine::{
        combat::{card::Cards, npc::Npcs},
        core::{
            location::Location,
            state::State,
            tag::{Static, TagKey},
        }, state::combat_state_machine::CombatStateMachine,
    };

pub static PLAYER_HAND: Static<TagKey> = Static::new(|| "player:hand".into());
pub static PLAYER_DECK: Static<TagKey> = Static::new(|| "player:deck".into());
pub static ENEMY_HAND: Static<TagKey> = Static::new(|| "enemy:hand".into());
pub static ENEMY_DECK: Static<TagKey> = Static::new(|| "enemy:deck".into());

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
            .expect("Failed to find location in world")
    }
}

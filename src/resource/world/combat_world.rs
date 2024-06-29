use std::collections::BTreeMap;

use serde::Serialize;

use crate::{
    engine::{state_machine::combat_state_machine::CombatStateMachine, tag_engine::TagEngine},
    resource::{
        combat::{card::Cards, npc::Npcs},
        core::{location::Location, state::State},
    },
};

type StateFn = dyn Fn(&CombatStateMachine, &TagEngine) -> State + Send + Sync;

pub struct DynamicStateFn {
    pub func: Box<StateFn>,
}

impl DynamicStateFn {
    pub fn apply(&self, machine: &CombatStateMachine, tag_engine: &TagEngine) -> State {
        (self.func)(machine, tag_engine)
    }
}

impl DynamicStateFn {
    pub fn new(func: fn(&CombatStateMachine, &TagEngine) -> State) -> Self {
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

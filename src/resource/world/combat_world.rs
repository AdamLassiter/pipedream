use std::collections::BTreeMap;

use crate::{
    engine::tag_engine::TagEngine,
    resource::{location::Location, state::State},
};

type StateFn = dyn Fn(&TagEngine) -> State + Send + Sync;

pub struct DynamicStateFn(Box<StateFn>);

impl DynamicStateFn {
    pub fn apply(&self, tag_engine: &TagEngine) -> State {
        self.0(tag_engine)
    }
}

impl DynamicStateFn {
    pub fn new(func: fn(&TagEngine) -> State) -> Self {
        Self(Box::new(func))
    }
}

pub struct CombatWorld {
    pub states: BTreeMap<Location, DynamicStateFn>,
}

impl CombatWorld {
    pub fn get_state(&self, location: &Location) -> &DynamicStateFn {
        self.states.get(location).unwrap()
    }
}

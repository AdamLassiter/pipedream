use std::collections::BTreeMap;

use crate::{
    engine::tag_engine::TagEngine,
    resource::{location::Location, state::State},
};

pub trait DynamicWorld: Sized {
    fn get_state(&self, location: &Location) -> &DynamicStateFn;
}

type StateFn = dyn Fn(&TagEngine) -> State;

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
    states: BTreeMap<Location, DynamicStateFn>,
}

impl DynamicWorld for CombatWorld {
    fn get_state(&self, location: &Location) -> &DynamicStateFn {
        self.states.get(location).unwrap()
    }
}

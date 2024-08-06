use rusqlite_orm::orm_bind;
use serde::{Deserialize, Serialize};

use super::choice::Choices;

use super::state_machine::StateMachine;
use super::{location::Location, scene::Scene};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[orm_bind {location: "$.location.location"}]
pub struct State {
    pub location: Location,
    pub scene: Scene,
    pub choices: Choices,
}

type StateFn = dyn Fn(&StateMachine) -> State + Send + Sync;

pub struct DynamicStateFn {
    pub func: Box<StateFn>,
}

impl DynamicStateFn {
    pub fn apply(&self, machine: &StateMachine) -> State {
        (self.func)(machine)
    }
}

impl DynamicStateFn {
    pub fn new(func: fn(&StateMachine) -> State) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

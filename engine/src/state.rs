use rusqlite_orm::orm_autobind;

use crate::command::UiMode;

use super::choice::Choices;

use super::state_machine::StateMachine;
use super::scene::Scene;

#[derive(Clone, Debug)]
#[orm_autobind]
pub struct State {
    pub location: String,
    pub ui_mode: UiMode,
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

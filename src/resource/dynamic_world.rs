use crate::{engine::dynamic_state_machine::DynamicStateMachine, resource::state::State};

use super::location::Location;

pub trait DynamicWorld: Sized {
    fn get_state(&self, location: &Location) -> &DynamicStateFn<Self>;
}

type StateFnT<W> = dyn Fn(&DynamicStateMachine<W>) -> State;
pub struct DynamicStateFn<W: DynamicWorld>(Box<StateFnT<W>>);

impl<W: DynamicWorld> DynamicStateFn<W> {
    pub fn apply(&self, state_machine: &DynamicStateMachine<W>) -> State {
        self.0(state_machine)
    }
}

impl<W: 'static + DynamicWorld> DynamicStateFn<W> {
    pub fn new(func: fn(&DynamicStateMachine<W>) -> State) -> Self {
        Self(Box::new(func))
    }
}

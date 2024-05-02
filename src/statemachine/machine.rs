use std::collections::HashMap;

use crate::resource::location::Location;

use super::state::State;

#[derive(Debug)]
pub struct Machine<S, A> {
    pub states: HashMap<Location, State<S, A>>,
    pub current: Location,
}

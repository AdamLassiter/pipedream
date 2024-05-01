use std::collections::HashMap;

use super::{location::Location, state::State};

pub struct Machine {
    pub states: HashMap<Location, State>,
    pub current: Location,
}

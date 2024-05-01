use std::collections::HashMap;

use crate::interface::{options::Options, scene::Scene};

use super::location::Location;

pub struct State {
    pub location: String,
    pub scene: Scene,
    pub transitions: HashMap<Options, Location>,
}

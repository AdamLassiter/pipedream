use serde_derive::Serialize;

use crate::interface::choices::Choices;

use super::{location::Location, scene::Scene};

#[derive(Debug, Serialize)]
pub struct State {
    pub location: Location,
    pub scene: Scene,
    pub options: Choices,
}

impl From<State> for (Location, State) {
    fn from(value: State) -> Self {
        (value.location.clone(), value)
    }
}

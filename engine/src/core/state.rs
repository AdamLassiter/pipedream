use serde::{Deserialize, Serialize};

use super::choice::Choices;

use super::{location::Location, scene::Scene};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub location: Location,
    pub scene: Scene,
    #[serde(flatten)]
    pub options: Choices,
}

impl From<State> for (Location, State) {
    fn from(value: State) -> Self {
        (value.location.clone(), value)
    }
}

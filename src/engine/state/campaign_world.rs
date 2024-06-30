use crate::engine::core::{location::Location, state::State};

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignWorld {
    pub states: BTreeMap<Location, State>,
}

impl CampaignWorld {
    pub fn get_state(&self, location: &Location) -> &State {
        self.states.get(location).expect("Failed to find location in world")
    }
}

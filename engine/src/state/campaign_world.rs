use crate::core::{location::Location, state::State};

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignWorld {
    pub states: BTreeMap<Location, State>,
}

impl From<Vec<State>> for CampaignWorld {
    fn from(values: Vec<State>) -> Self {
        Self {
            states: BTreeMap::from_iter(
                values
                    .into_iter()
                    .map(|state| (state.location.clone(), state)),
            ),
        }
    }
}

impl CampaignWorld {
    pub fn get_state(&self, location: &Location) -> &State {
        self.states
            .get(location)
            .unwrap_or_else(|| panic!("Failed to find location {:?} in campaign world", location.0))
    }
}

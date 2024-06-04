use crate::resource::{location::Location, state::State};

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub trait StaticWorld {
    fn get_state(&self, location: &Location) -> &State;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignWorld {
    pub(crate) states: BTreeMap<Location, State>,
}

impl StaticWorld for CampaignWorld {
    fn get_state(&self, location: &Location) -> &State {
        self.states.get(location).unwrap()
    }
}

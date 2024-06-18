use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::{
    engine::{state_machine::campaign_state_machine::CampaignStateMachine, tag_engine::TagEngine},
    resource::core::{commands::UiCommand, location::Location, transition::Transition},
};

use super::Coordinator;

#[derive(Serialize, Deserialize)]
pub struct CampaignCoordinator {
    pub start: Location,
    pub tag_engine: TagEngine,
    pub state_machine: CampaignStateMachine,
}

impl Coordinator for CampaignCoordinator {
    fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand> {
        self.state_machine
            .handle_effect(&mut self.tag_engine, side_effect)
    }

    fn dump(&self) {
        let buffer = File::create("./campaign-coordinator-state.yaml").unwrap();
        serde_yml::to_writer(buffer, &self).unwrap();
    }
}

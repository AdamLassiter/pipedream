use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::{
    engine::{state_machine::{static_state_machine::StaticStateMachine, StateMachine}, tag_engine::TagEngine},
    resource::{commands::UiCommand, location::Location, transition::Transition, world::static_world::CampaignWorld},
};

use super::Coordinator;

#[derive(Serialize, Deserialize)]
pub struct CampaignCoordinator {
    pub start: Location,
    pub tag_engine: TagEngine,
    pub state_machine: StaticStateMachine<CampaignWorld>,
}

impl Coordinator for CampaignCoordinator {
    fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand> {
        self.state_machine
            .handle_effect(&mut self.tag_engine, side_effect)
    }

    fn dump(&self) {
        let buffer = File::create("./campaign-state.yaml").unwrap();
        serde_yaml::to_writer(buffer, &self).unwrap();
    }
}

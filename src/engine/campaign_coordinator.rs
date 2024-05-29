use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::{
    engine::{static_state_machine::StaticStateMachine, tag_engine::TagEngine},
    resource::{
        commands::UiCommand, location::Location, prefab::campaign_world::CampaignWorld,
        transition::Transition,
    },
};

#[derive(Serialize, Deserialize)]
pub struct CampaignCoordinator {
    pub start: Location,
    pub tag_engine: TagEngine,
    pub state_machine: StaticStateMachine<CampaignWorld>,
}

impl CampaignCoordinator {
    pub fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand> {
        self.state_machine
            .handle_effect(&mut self.tag_engine, side_effect)
    }

    pub fn dump(&self) {
        let buffer = File::create("./campaign-state.yaml").unwrap();
        serde_yaml::to_writer(buffer, &self).unwrap();
    }
}

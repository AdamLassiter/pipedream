use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::{
    engine::{state_machine::StateMachine, tag_engine::TagEngine},
    resource::{
        commands::UiCommand, location::Location, transition::SideEffect, world::CampaignWorld,
    },
};

#[derive(Serialize, Deserialize)]
pub struct Campaign {
    pub start: Location,
    pub tag_engine: TagEngine,
    pub state_machine: StateMachine<CampaignWorld>,
}

impl Campaign {
    pub fn handle_effect(&mut self, side_effect: SideEffect) -> Vec<UiCommand> {
        self.state_machine
            .handle_effect(&mut self.tag_engine, side_effect)
    }

    pub fn dump(&self) {
        let buffer = File::create("./campaign-state.yaml").unwrap();
        serde_yaml::to_writer(buffer, &self).unwrap();
    }
}

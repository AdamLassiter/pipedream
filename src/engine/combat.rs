use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::{
    engine::{state_machine::StateMachine, tag_engine::TagEngine},
    resource::{commands::UiCommand, location::Location, transition::SideEffect},
};

#[derive(Serialize, Deserialize)]
pub struct Combat {
    pub tag_engine: TagEngine,
    pub state_machine: StateMachine,
}

impl Combat {
    pub fn handle_effect(&mut self, side_effect: SideEffect) -> Vec<UiCommand> {
    }

    pub fn dump(&self) {
        let buffer = File::create("./combat-state.yaml").unwrap();
        serde_yaml::to_writer(buffer, &self).unwrap();
    }
}

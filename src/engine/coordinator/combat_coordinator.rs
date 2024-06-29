use std::fs::File;

use serde::Serialize;

use crate::{
    engine::{state_machine::combat_state_machine::CombatStateMachine, tag_engine::TagEngine},
    resource::core::{commands::UiCommand, transition::Transition},
};

use super::Coordinator;

#[derive(Serialize)]
pub struct CombatCoordinator {
    pub tag_engine: TagEngine,
    pub state_machine: CombatStateMachine,
}

impl Coordinator for CombatCoordinator {
    fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand> {
        self.state_machine
            .handle_effect(&mut self.tag_engine, side_effect)
    }

    fn dump(&self) {
        let buffer = File::create("./combat-coordinator.yml")
            .expect("Failed to open file for writing combat-coordinator data");
        serde_yml::to_writer(buffer, &self)
            .expect("Failed to write yaml combat-coordinator data to file");
    }
}

use std::fs::File;

use serde::Serialize;

use crate::{
    engine::{
        state_machine::{dynamic_state_machine::DynamicStateMachine, StateMachine},
        tag_engine::TagEngine,
    },
    prefab::combat_world::CombatWorld,
    resource::{combat::field::CombatEntity, commands::UiCommand, transition::Transition},
};

use super::Coordinator;

#[derive(Serialize)]
pub struct CombatCoordinator {
    pub tag_engine: TagEngine,
    pub player: CombatEntity,
    pub enemy: CombatEntity,
    #[serde(skip_serializing)]
    pub state_machine: DynamicStateMachine<CombatWorld>,
}

impl Coordinator for CombatCoordinator {
    fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand> {
        self.state_machine
            .handle_effect(&mut self.tag_engine, side_effect)
    }

    fn dump(&self) {
        let buffer = File::create("./combat-state.yaml").unwrap();
        serde_yaml::to_writer(buffer, &self).unwrap();
    }
}

use std::fs::File;

use serde::Serialize;

use crate::{
    engine::tag_engine::TagEngine,
    resource::{combat::field::CombatEntity, prefab::combat_world::CombatWorld},
};

use super::dynamic_state_machine::DynamicStateMachine;

#[derive(Serialize)]
pub struct CombatCoordinator {
    pub tag_engine: TagEngine,
    pub player: CombatEntity,
    pub enemy: CombatEntity,
    #[serde(skip_serializing)]
    pub state_machine: DynamicStateMachine<CombatWorld>,
}

impl CombatCoordinator {
    pub fn dump(&self) {
        let buffer = File::create("./combat-state.yaml").unwrap();
        serde_yaml::to_writer(buffer, &self).unwrap();
    }
}

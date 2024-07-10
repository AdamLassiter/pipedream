use std::time::Duration;

use pipedream_engine::{
    core::{
        scene::Scene,
        state::State,
        transition::{Transition, TransitionType},
    },
    state::combat_state_machine::CombatStateMachine,
};

use crate::combat_world::COMBAT_DEFEAT;

pub fn combat_defeat(_machine: &CombatStateMachine) -> State {
    State {
        location: COMBAT_DEFEAT.clone(),
        scene: Scene {
            descriptions: vec!["Defeat!".into()],
        },
        options: (
            Transition {
                next: TransitionType::Leave,
                actions: vec![],
            },
            Duration::from_secs(2),
        )
            .into(),
    }
}

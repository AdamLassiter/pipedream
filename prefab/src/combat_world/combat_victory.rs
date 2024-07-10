use std::time::Duration;

use pipedream_engine::{
    core::{
        scene::Scene,
        state::State,
        transition::{Transition, TransitionType},
    },
    state::combat_state_machine::CombatStateMachine,
};

use crate::combat_world::COMBAT_VICTORY;

pub fn combat_victory(_machine: &CombatStateMachine) -> State {
    State {
        location: COMBAT_VICTORY.clone(),
        scene: Scene {
            descriptions: vec!["Victory!".into()],
        },
        options: (
            Transition {
                next: TransitionType::Leave,
                actions: vec![],
            },
            Duration::from_secs(2),
        ).into(),
    }
}

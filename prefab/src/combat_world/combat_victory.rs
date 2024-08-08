use std::time::Duration;

use pipedream_engine::{
    core::{
        choice::Choices,
        effect::{Effect, Transition},
        scene::Scene,
        state::State,
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
        choices: Choices::timed(
            Effect {
                transition: Transition::Leave,
                actions: vec![],
            },
            Duration::from_secs(2),
        ),
    }
}

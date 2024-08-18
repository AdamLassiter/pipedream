use std::time::Duration;

use pipedream_engine::core::{
        choice::Choices, description::Description, effect::{Effect, Transition}, scene::Scene, state::State, state_machine::StateMachine
    };

use crate::combat_world::COMBAT_VICTORY;

pub fn combat_victory(_machine: &StateMachine) -> State {
    State {
        location: COMBAT_VICTORY.clone(),
        scene: Scene {
            descriptions: vec![Description::always("Victory!")],
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

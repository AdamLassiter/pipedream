use std::time::Duration;

use pipedream_engine::{
    choice::Choices,
    description::Description,
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
    state_machine::StateMachine,
};

use crate::combat_world::COMBAT_DEFEAT;

pub fn combat_defeat(_machine: &StateMachine) -> State {
    State {
        location: COMBAT_DEFEAT.clone(),
        scene: Scene {
            descriptions: vec![Description::always("Defeat!")],
        },
        choices: Choices::timed(
            Effect {
                transition: Transition::Leave,
                ..Default::default()
            },
            Duration::from_secs(2),
        )
        .into(),
    }
}

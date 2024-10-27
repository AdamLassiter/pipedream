use std::time::Duration;

use pipedream_domain::{
    choice::Choices,
    description::Description,
    effect::{Effect, Transition},
};
use pipedream_engine::{command::UiMode, scene::Scene, state::State, state_machine::StateMachine};

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
                ..Default::default()
            },
            Duration::from_secs(2),
        ),
        ui_mode: UiMode::Combat,
    }
}

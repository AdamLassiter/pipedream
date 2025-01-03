use pipedream_domain::{
    choice::Choices,
    description::Description,
    effect::{Effect, Transition},
};
use pipedream_engine::{command::UiMode, scene::Scene, state::State, state_machine::StateMachine};

use crate::{COMBAT_ADVANCE_TIME, combat_world::COMBAT_DEFEAT};

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
            COMBAT_ADVANCE_TIME,
        ),
        ui_mode: UiMode::Combat,
    }
}

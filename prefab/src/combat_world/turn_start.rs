use pipedream_domain::{
    choice::Choices,
    effect::{Effect, Transition},
    player::Player,
};
use pipedream_engine::{command::UiMode, scene::Scene, state::State, state_machine::StateMachine};

use super::{CPU_DRAW, CPU_START, HUMAN_DRAW, HUMAN_START};

pub fn turn_start(player: &Player, _machine: &StateMachine) -> State {
    let current_location = match player {
        Player::Human => HUMAN_START.clone(),
        Player::Cpu => CPU_START.clone(),
        Player::World => unimplemented!(),
    };
    let next_location = match player {
        Player::Human => HUMAN_DRAW.clone(),
        Player::Cpu => CPU_DRAW.clone(),
        Player::World => unimplemented!(),
    };
    State {
        location: current_location,
        scene: Scene {
            descriptions: vec![],
        },
        choices: Choices::skip(Effect::transition(Transition::Goto(next_location))),
        ui_mode: UiMode::Combat,
    }
}

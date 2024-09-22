use std::time::Duration;

use log::debug;

use crate::combat_world::{COMBAT_INIT, HUMAN_DRAW};
use pipedream_domain::{player::Player, player::PlayerCharacter};
use pipedream_engine::{
    choice::Choices,
    command::UiMode,
    description::Description,
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
    state_machine::StateMachine,
};

pub fn combat_init(machine: &StateMachine) -> State {
    let (_, cpu) = PlayerCharacter::get_player_character(&machine.conn, &Player::Cpu);
    debug!(target:"Prefab/Combat/Init", "{:?}", cpu.name);

    State {
        location: COMBAT_INIT.clone(),
        scene: Scene {
            descriptions: vec![
                Description::always("A challenger appears!"),
                Description::always(format!("{:?} is looking for a fight", cpu.name)),
            ],
        },
        choices: Choices::timed(
            Effect {
                transition: Transition::Goto(HUMAN_DRAW.clone()),
                ..Default::default()
            },
            Duration::from_secs(2),
        ),
        ui_mode: UiMode::Combat,
    }
}

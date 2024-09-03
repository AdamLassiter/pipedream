use std::time::Duration;

use log::debug;
use pipedream_domain::{character::Character, encounter::Player};
use pipedream_engine::{choice::Choices, description::Description, state_machine::StateMachine};

use crate::combat_world::{COMBAT_INIT, HUMAN_DRAW};
use pipedream_engine::{
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
};

pub fn combat_init(machine: &StateMachine) -> State {
    let cpu = Character::get_player(&machine.conn, &Player::Cpu);
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
    }
}

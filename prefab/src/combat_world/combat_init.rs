use std::time::Duration;

use pipedream_engine::{
    core::{choice::Choices, description::Description, state_machine::StateMachine},
    domain::encounter::Player,
};
use log::debug;

use crate::combat_world::{COMBAT_INIT, HUMAN_DRAW};
use pipedream_engine::core::{
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
};

pub fn combat_init(machine: &StateMachine) -> State {
    let cpu = machine.get_character(&Player::Cpu);
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
                actions: vec![],
            },
            Duration::from_secs(2),
        ),
    }
}

use pipedream_engine::{
    core::{
        choice::Choices,
        effect::{Effect, Transition},
        scene::Scene,
        state::State,
        state_machine::StateMachine,
    },
    domain::{encounter::Player, stats::Resource},
};
use log::debug;

use crate::combat_world::{COMBAT_DEFEAT, COMBAT_END, COMBAT_VICTORY, HUMAN_PLAY};

pub fn combat_end(machine: &StateMachine) -> State {
    let human = machine.get_character(&Player::Human);
    let cpu = machine.get_character(&Player::Cpu);

    debug!(target:"Prefab/Combat/End", "{:?} vs {:?}", human, cpu);

    let next = if *human
        .stats
        .resources
        .get(&Resource::Health)
        .expect("Failed to find Human health")
        <= 0
    {
        &COMBAT_DEFEAT
    } else if *cpu
        .stats
        .resources
        .get(&Resource::Health)
        .expect("Failed to find Cpu health")
        <= 0
    {
        &COMBAT_VICTORY
    } else {
        &HUMAN_PLAY
    };

    State {
        location: COMBAT_END.clone(),
        scene: Scene {
            descriptions: vec![],
        },
        choices: Choices::skip(Effect {
            transition: Transition::Goto((*next).clone()),
            actions: vec![],
        }),
    }
}

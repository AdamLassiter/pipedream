use log::debug;
use pipedream_domain::{character::Character, encounter::Player, stats::Resource};
use pipedream_engine::{
    choice::Choices,
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
    state_machine::StateMachine,
};

use crate::combat_world::{COMBAT_DEFEAT, COMBAT_END, COMBAT_VICTORY, HUMAN_PLAY};

pub fn combat_end(machine: &StateMachine) -> State {
    let human = Character::get_player(&machine.conn, &Player::Human);
    let cpu = Character::get_player(&machine.conn, &Player::Cpu);

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
            ..Default::default()
        }),
    }
}

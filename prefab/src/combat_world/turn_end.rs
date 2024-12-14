use log::debug;
use pipedream_domain::choice::Choices;
use pipedream_domain::effect::{Effect, Transition};
use pipedream_domain::player::{Player, PlayerCharacter};
use pipedream_domain::stats::Resource;
use pipedream_domain::target::{Target, TargetCharacter};
use pipedream_engine::scene::Scene;
use pipedream_engine::{command::UiMode, state_machine::StateMachine};

use pipedream_engine::state::State;

use crate::combat_world::{COMBAT_DEFEAT, COMBAT_VICTORY};

use super::{CPU_END, CPU_START, HUMAN_END, HUMAN_START};

pub fn turn_end(player: &Player, machine: &StateMachine) -> State {
    let (_, human) = PlayerCharacter::get_player_character(&machine.conn, &Player::Human);
    let (_, cpu) = PlayerCharacter::get_player_character(&machine.conn, &Player::Cpu);

    debug!(target:"Prefab/Turn/End", "{:?} vs {:?}", human, cpu);

    let current_location = match player {
        Player::Human => HUMAN_END.clone(),
        Player::Cpu => CPU_END.clone(),
        Player::World => unimplemented!(),
    };

    let next_location = if *human
        .stats
        .resources
        .get(&Resource::Health)
        .expect("Failed to find Human health")
        <= 0
    {
        COMBAT_DEFEAT.clone()
    } else if *cpu
        .stats
        .resources
        .get(&Resource::Health)
        .expect("Failed to find Cpu health")
        <= 0
    {
        COMBAT_VICTORY.clone()
    } else {
        match player {
            Player::Human => CPU_START.clone(),
            Player::Cpu => HUMAN_START.clone(),
            Player::World => unimplemented!(),
        }
    };

    let actions = if next_location == *CPU_START || next_location == *CPU_END {
        vec![
            TargetCharacter::update_action(&machine.conn, &Target::Me, |mut target_char| {
                target_char.target = Target::You;
                target_char
            }),
            TargetCharacter::update_action(&machine.conn, &Target::You, |mut target_char| {
                target_char.target = Target::Me;
                target_char
            }),
        ]
    } else {
        vec![]
    };

    State {
        location: current_location,
        ui_mode: UiMode::Combat,
        scene: Scene {
            descriptions: vec![],
        },
        choices: Choices::skip(Effect {
            transition: Transition::Goto(next_location),
            actions,
        }),
    }
}

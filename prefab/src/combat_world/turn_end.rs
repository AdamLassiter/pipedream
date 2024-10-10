use pipedream_domain::choice::Choices;
use pipedream_domain::effect::{Effect, Transition};
use pipedream_domain::player::Player;
use pipedream_domain::target::{Target, TargetCharacter};
use pipedream_engine::scene::Scene;
use pipedream_engine::{command::UiMode, state_machine::StateMachine};

use pipedream_engine::state::State;

use super::{CPU_END, CPU_START, HUMAN_END, HUMAN_START};

pub fn turn_end(player: &Player, machine: &StateMachine) -> State {
    let swap_targets = vec![
        TargetCharacter::update_action(&machine.conn, &Target::Me, |mut target_char| {
            target_char.target = Target::You;
            target_char
        }),
        TargetCharacter::update_action(&machine.conn, &Target::You, |mut target_char| {
            target_char.target = Target::Me;
            target_char
        }),
    ];

    let current_location = match player {
        Player::Human => HUMAN_END.clone(),
        Player::Cpu => CPU_END.clone(),
        Player::World => panic!("No location for World"),
    };
    let next_location = match player {
        Player::Human => CPU_START.clone(),
        Player::Cpu => HUMAN_START.clone(),
        Player::World => panic!("No location for World"),
    };

    State {
        location: current_location,
        ui_mode: UiMode::Combat,
        scene: Scene {
            descriptions: vec![],
        },
        choices: Choices::skip(Effect {
            transition: Transition::Goto(next_location),
            actions: swap_targets,
        }),
    }
}

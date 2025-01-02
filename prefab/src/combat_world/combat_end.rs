use log::debug;
use pipedream_domain::action::Action;
use pipedream_domain::card::PlacedCard;
use pipedream_domain::choice::Choices;
use pipedream_domain::effect::{Effect, Transition};
use pipedream_domain::player::{Player, PlayerCharacter, PlayerCharacterDao};
use pipedream_domain::stats::Resource;
use pipedream_domain::target::TargetCharacter;
use pipedream_engine::scene::Scene;
use pipedream_engine::{command::UiMode, state_machine::StateMachine};

use pipedream_engine::state::State;

use crate::combat_world::{COMBAT_DEFEAT, COMBAT_END, COMBAT_VICTORY};

pub fn combat_end(machine: &StateMachine) -> State {
    let (human_id, human) = PlayerCharacter::get_player_character(&machine.conn, &Player::Human);
    let (_, cpu) = PlayerCharacter::get_player_character(&machine.conn, &Player::Cpu);

    debug!(target:"Prefab/Combat/End", "{:?} vs {:?}", human, cpu);

    let current_location = COMBAT_END.clone();

    let human_health = *human
        .stats
        .resources
        .get(&Resource::Health)
        .expect("Failed to find Human health");
    let cpu_health = *cpu
        .stats
        .resources
        .get(&Resource::Health)
        .expect("Failed to find Cpu health");

    let next_location = if human_health <= 0 {
        COMBAT_DEFEAT.clone()
    } else if cpu_health <= 0 {
        COMBAT_VICTORY.clone()
    } else {
        unreachable!("Combat should not end if neither Defeat nor Victory")
    };

    let player_characters = PlayerCharacterDao::table_name();
    let recreate_human = format!(
        "insert into {player_characters} (player, character)
        values (:player, :character);"
    );
    let params = vec![
        (":player", serde_json::to_string(&Player::Human)),
        (":character", serde_json::to_string(&human_id)),
    ];

    let actions = [
        TargetCharacter::delete_target_characters(),
        PlacedCard::delete_placed_cards(),
        PlayerCharacter::delete_player_characters(),
        vec![Action::parameterised(recreate_human, params)],
    ]
    .concat();

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

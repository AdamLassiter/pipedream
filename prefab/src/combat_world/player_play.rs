use log::debug;

use crate::combat_world::{CPU_DAMAGE, CPU_PLAY, HUMAN_DAMAGE, HUMAN_PLAY};
use pipedream_domain::{
    card::{Card, PlacedCard},
    choice::{Choice, Choices},
    description::Description,
    effect::{Effect, Transition},
    field::FieldPlace,
    player::Player,
    player::PlayerCharacter,
};
use pipedream_engine::{command::UiMode, scene::Scene, state::State, state_machine::StateMachine};

pub fn player_play(player: &Player, machine: &StateMachine) -> State {
    let (character_id, _character) = PlayerCharacter::get_player_character(&machine.conn, player);
    let player_hand = PlacedCard::get_placed_cards(&machine.conn, &character_id, &FieldPlace::Hand);
    debug!(target:"Prefab/Combat/Hand", "{:?}", player_hand);

    let current_location = match player {
        Player::Human => HUMAN_PLAY.clone(),
        Player::Cpu => CPU_PLAY.clone(),
        Player::World => panic!("No location for World"),
    };
    let next_location = match player {
        Player::Human => HUMAN_DAMAGE.clone(),
        Player::Cpu => CPU_DAMAGE.clone(),
        Player::World => panic!("No location for World"),
    };

    State {
        location: current_location,
        scene: Scene {
            descriptions: vec![Description::always("Play")],
        },
        choices: Choices::manual(
            player_hand
                .into_iter()
                .flat_map(|(_id, PlacedCard { card: card_id, .. })| {
                    Card::get_card(&machine.conn, &card_id).into_iter()
                })
                .map(|card| {
                    let selectable = card.choice.predicate_satisfied(&machine.conn);
                    Choice {
                        effect: Effect {
                            transition: Transition::Goto(next_location.clone()),
                            ..card.choice.effect
                        },
                        selectable,
                        ..card.choice
                    }
                })
                .collect::<Vec<_>>(),
        ),
        ui_mode: UiMode::Combat,
    }
}

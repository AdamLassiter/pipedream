use std::time::Duration;

use log::debug;
use strum::IntoEnumIterator;

use crate::combat_world::{COMBAT_START, HUMAN_DRAW};
use pipedream_domain::{
    card::{Card, PlacedCard},
    choice::Choices,
    description::Description,
    effect::{Effect, Transition},
    player::{Player, PlayerCharacter},
    target::{Target, TargetCharacter},
};
use pipedream_engine::{command::UiMode, scene::Scene, state::State, state_machine::StateMachine};

pub fn combat_start(machine: &StateMachine) -> State {
    let mut enemy_name = None;

    Player::iter()
        .filter(|player| *player != Player::World)
        .for_each(|player| {
            let (character_id, character) =
                PlayerCharacter::get_player_character(&machine.conn, &player);

            debug!(target:"Prefab/Combat/Init", "{:?} {:?}", player, character.name);
            match player {
                Player::Human => {
                    TargetCharacter::insert_target_character(
                        &machine.conn,
                        TargetCharacter {
                            target: Target::Me,
                            character: character_id,
                        },
                    );
                }
                Player::Cpu => {
                    TargetCharacter::insert_target_character(
                        &machine.conn,
                        TargetCharacter {
                            target: Target::Me,
                            character: character_id,
                        },
                    );
                    enemy_name = Some(character.name);
                }
                Player::World => {
                    unreachable!();
                }
            }

            PlacedCard::insert_placed_cards(
                &machine.conn,
                character
                    .cards
                    .iter()
                    .map(|card_id| {
                        let card = Card::get_card(&machine.conn, card_id)
                            .unwrap_or_else(|| panic!("Failed to find card {}", card_id.0));
                        PlacedCard {
                            character: character_id,
                            card: *card_id,
                            place: card.starts,
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        });

    State {
        location: COMBAT_START.clone(),
        scene: Scene {
            descriptions: vec![
                Description::always("A challenger appears!"),
                Description::always(format!(
                    "{:?} is looking for a fight",
                    enemy_name.unwrap_or("Nobody".to_string())
                )),
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

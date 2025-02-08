use log::debug;
use rusqlite::Connection;

use crate::{
    combat_world::{CPU_DAMAGE, CPU_END, CPU_PLAY, HUMAN_DAMAGE, HUMAN_END, HUMAN_PLAY},
    states::message,
};
use pipedream_domain::{
    card::{Card, PlacedCard},
    choice::{Choice, Choices},
    description::Description,
    effect::{Effect, Transition},
    field::FieldPlace,
    player::{Player, PlayerCharacter},
};
use pipedream_engine::{command::UiMode, scene::Scene, state::State, state_machine::StateMachine};

pub fn player_play(player: &Player, machine: &StateMachine) -> State {
    let colour = match player {
        Player::Human => "yellow",
        Player::Cpu => "red",
        Player::World => "green",
    };
    let (character_id, character) = PlayerCharacter::get_player_character(&machine.conn, player);
    let player_hand = PlacedCard::get_placed_cards(&machine.conn, &character_id, &FieldPlace::Hand);
    debug!(target:"Prefab/Combat/Hand", "{:?}", player_hand);

    let current_location = match player {
        Player::Human => HUMAN_PLAY.clone(),
        Player::Cpu => CPU_PLAY.clone(),
        Player::World => unimplemented!(),
    };
    let next_location = match player {
        Player::Human => HUMAN_DAMAGE.clone(),
        Player::Cpu => CPU_DAMAGE.clone(),
        Player::World => unimplemented!(),
    };
    let end_turn_location = match player {
        Player::Human => HUMAN_END.clone(),
        Player::Cpu => CPU_END.clone(),
        Player::World => unimplemented!(),
    };

    let skip_card = match player {
        Player::Human => human_skip(&machine.conn),
        Player::Cpu => cpu_skip(&machine.conn),
        Player::World => unimplemented!(),
    };
    let system_choices = vec![skip_card.clone()].into_iter().map(|card| card.choice);

    let mut player_has_choices = false;
    let player_choices = player_hand
        .into_iter()
        .flat_map(|(_id, PlacedCard { card: card_id, .. })| {
            Card::get_card(&machine.conn, &card_id).into_iter()
        })
        .map(|card| {
            let selectable = card.choice.predicate_satisfied(&machine.conn);
            player_has_choices |= selectable;
            Choice {
                effect: Effect {
                    transition: Transition::Goto(next_location.clone()),
                    actions: [
                        card.choice.effect.actions,
                        vec![message(format!(
                            "<{} {}> played <blue {}>",
                            colour, character.name, card.choice.title
                        ))],
                    ]
                    .concat(),
                },
                selectable,
                ..card.choice
            }
        })
        .chain(system_choices)
        .collect::<Vec<_>>();

    match (player, player_has_choices) {
        (Player::Human, true) => State {
            location: current_location,
            scene: Scene {
                descriptions: vec![Description::always("Play")],
            },
            choices: Choices::manual(player_choices),
            ui_mode: UiMode::Combat,
        },
        (Player::Cpu, true) => State {
            location: current_location,
            scene: Scene {
                descriptions: vec![Description::always("Play")],
            },
            choices: Choices::cpu(player_choices, skip_card.choice),
            ui_mode: UiMode::Combat,
        },
        (Player::World, _) => {
            unimplemented!()
        }
        (_, false) => State {
            location: current_location,
            scene: Scene {
                descriptions: vec![Description::always("Play")],
            },
            choices: Choices::skip(Effect::transition(Transition::Goto(end_turn_location))),
            ui_mode: UiMode::Combat,
        },
    }
}

fn human_skip(conn: &Connection) -> Card {
    Card::get_card_title(conn, &"Skip".into())
        .map(|(_, card)| card)
        .expect("Failed to find Human Skip")
}

fn cpu_skip(conn: &Connection) -> Card {
    Card::get_card_title(conn, &"Exhaust".into())
        .map(|(_, card)| card)
        .expect("Failed to find Cpu Exhaust")
}

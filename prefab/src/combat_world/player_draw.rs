use std::time::Duration;

use log::debug;

use crate::combat_world::{HUMAN_DRAW, HUMAN_PLAY};
use pipedream_domain::{
    card::PlacedCard,
    choice::Choices,
    description::Description,
    effect::{Effect, Transition},
    field::FieldPlace,
    player::Player,
    player::PlayerCharacter,
    stats::SleightOfHand,
};
use pipedream_engine::{command::UiMode, scene::Scene, state::State, state_machine::StateMachine};

use super::{CPU_DRAW, CPU_PLAY};

pub fn player_draw(player: &Player, machine: &StateMachine) -> State {
    let (character_id, character) = PlayerCharacter::get_player_character(&machine.conn, player);
    let draw_count = character
        .stats
        .sleight_of_hand
        .get(&SleightOfHand::Inspiration)
        .expect("Failed to find Player Inspration");

    let deck_draw = PlacedCard::update_placed_cards(
        &machine.conn,
        &character_id,
        &FieldPlace::Deck,
        |mut deck| {
            draw_cards(&mut deck, *draw_count);
            deck
        },
    );

    let current_location = match player {
        Player::Human => HUMAN_DRAW.clone(),
        Player::Cpu => CPU_DRAW.clone(),
        Player::World => unimplemented!(),
    };
    let next_location = match player {
        Player::Human => HUMAN_PLAY.clone(),
        Player::Cpu => CPU_PLAY.clone(),
        Player::World => unimplemented!(),
    };

    State {
        location: current_location,
        scene: Scene {
            descriptions: vec![Description::always("Draw!")],
        },
        choices: Choices::timed(
            Effect {
                transition: Transition::Goto(next_location),
                actions: deck_draw,
            },
            Duration::from_secs(2),
        ),
        ui_mode: UiMode::Combat,
    }
}

fn draw_cards(deck: &mut Vec<PlacedCard>, draw_count: u16) {
    // player_deck_slice.shuffle(&mut thread_rng());
    deck.iter_mut().take(draw_count as usize).for_each(|card| {
        card.place = FieldPlace::Hand;
    });

    debug!(target:"Prefab/Combat/Draw", "{:?}", deck);
}

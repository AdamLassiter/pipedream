use std::{cell::RefCell, time::Duration};

use crate::combat_world::{HUMAN_DRAW, HUMAN_PLAY};
use log::debug;
use pipedream_domain::{
    card::PlacedCard, character::PlayerCharacter, field::FieldPlace, player::Player,
    stats::SleightOfHand,
};
use pipedream_engine::{
    choice::Choices,
    description::Description,
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
    state_machine::StateMachine,
};

pub fn player_draw(player: &Player, machine: &StateMachine) -> State {
    let (character_id, character) = PlayerCharacter::get_player_character(&machine.conn, player);
    let draw_count = character
        .stats
        .sleight_of_hand
        .get(&SleightOfHand::Inspiration)
        .expect("Failed to find Player Inspration");

    let cards_drawn = RefCell::new(vec![]);
    let deck_remove =
        PlacedCard::update_placed_cards(&machine.conn, &character_id, &FieldPlace::Deck, |mut deck| {
            let mut drawn_from_deck = draw_cards(&mut deck, *draw_count);
            cards_drawn.borrow_mut().append(&mut drawn_from_deck);
            deck
        });
    let hand_add =
        PlacedCard::update_placed_cards(&machine.conn, &character_id, &FieldPlace::Hand, |mut hand| {
            hand.append(&mut cards_drawn.borrow_mut());
            hand
        });

    State {
        location: HUMAN_DRAW.clone(),
        scene: Scene {
            descriptions: vec![Description::always("Draw!")],
        },
        choices: Choices::timed(
            Effect {
                transition: Transition::Goto(HUMAN_PLAY.clone()),
                actions: deck_remove
                    .into_iter()
                    .chain(hand_add.into_iter())
                    .collect::<Vec<_>>(),
            },
            Duration::from_secs(2),
        ),
    }
}

fn draw_cards(deck: &mut Vec<PlacedCard>, draw_count: u16) -> Vec<PlacedCard> {
    // player_deck_slice.shuffle(&mut thread_rng());
    let draw = deck.split_off(deck.len().saturating_sub(draw_count as usize));
    let draw = draw
        .into_iter()
        .map(|card| PlacedCard {
            place: FieldPlace::Hand,
            ..card
        })
        .collect::<Vec<_>>();

    debug!(target:"Prefab/Combat/Draw", "{:?}", draw);
    draw
}

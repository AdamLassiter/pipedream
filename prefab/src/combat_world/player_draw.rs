use std::{iter::repeat_n, time::Duration};

use crate::combat_world::{HUMAN_DRAW, HUMAN_PLAY};
use log::debug;
use pipedream_domain::{
    card::Card, character::Character, encounter::Player, stats::SleightOfHand, target::Target,
};
use pipedream_engine::{
    action::Action,
    choice::Choices,
    description::Description,
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
    state_machine::StateMachine,
    tag::Tag,
};
use rand::{prelude::SliceRandom, thread_rng};

pub fn player_draw(player: &Player, machine: &StateMachine) -> State {
    let mut player = Character::get_player(&machine.conn, player);
    let draw_count = player
        .stats
        .sleight_of_hand
        .get(&SleightOfHand::Inspiration)
        .expect("Failed to find Player Inspration");

    let player_draw_cards = draw_cards(&mut player.deck, draw_count);

    let player_hand_cards = player_draw_cards
        .iter()
        .map(|player_draw_card| {
            TagKey(player_draw_card.0.replace(
                String::from(Ent::Deck).as_str(),
                String::from(Ent::Hand).as_str(),
            ))
        })
        .collect::<Vec<_>>();

    State {
        location: HUMAN_DRAW.clone(),
        scene: Scene {
            descriptions: vec![Description::always("Draw!")],
        },
        choices: Choices::timed(
            Effect {
                transition: Transition::Goto(HUMAN_PLAY.clone()),
                action: player_draw_cards
                    .into_iter()
                    .map(|draw| Action::Subtract(draw.0.into()))
                    .chain(
                        player_hand_cards
                            .into_iter()
                            .map(|hand| Action::Add(hand.0.into())),
                    )
                    .collect(),
            },
            Duration::from_secs(2),
        ),
    }
}

fn draw_cards(deck: &mut Vec<Card>, draw_count: i16) -> Vec<Card> {
    // player_deck_slice.shuffle(&mut thread_rng());
    let draw = deck.split_off(deck.len().saturating_sub(draw_count as usize));

    debug!(target:"Prefab/Combat/Draw", "{:?}", draw);
    draw
}

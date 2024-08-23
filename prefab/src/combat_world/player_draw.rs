use std::{iter::repeat_n, time::Duration};

use crate::combat_world::{HUMAN_DRAW, HUMAN_PLAY};
use pipedream_engine::{
    core::{
        action::Action, choice::Choices, description::Description, effect::{Effect, Transition}, scene::Scene, state::State, state_machine::StateMachine, tag::Tag
    },
    domain::target::Target,
    log::debug,
    rand::{prelude::SliceRandom, thread_rng},
};

pub fn player_draw(machine: &StateMachine) -> State {
    let player_draw_count = machine
        .tag_engine
        .find(&Target::Me.ent(Ent::DrawCount))
        .iter()
        .filter_map(|tag| {
            if let TagValue::Number(n) = tag.value {
                Some(n)
            } else {
                None
            }
        })
        .next()
        .expect("Failed to find player draw count");

    let player_deck_slice = machine.tag_engine.find(&Target::Me.ent(Ent::Deck));
    let player_draw_cards = draw_cards(player_deck_slice, player_draw_count);

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
                actions: player_draw_cards
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

fn draw_cards(player_deck_slice: Vec<Tag>, player_draw_count: FI64) -> Vec<TagKey> {
    let mut player_deck_slice = player_deck_slice
        .iter()
        .flat_map(|tag| {
            repeat_n(
                tag.key.clone(),
                tag.value
                    .number()
                    .expect("Failed to get a Number of cards to draw")
                    .to_num(),
            )
        })
        .collect::<Vec<_>>();

    player_deck_slice.shuffle(&mut thread_rng());

    let player_draw_cards = player_deck_slice
        .into_iter()
        .take(player_draw_count.to_num())
        .collect::<Vec<_>>();

    debug!(target:"Prefab/Combat/Draw", "{:?}", player_draw_cards);
    player_draw_cards
}

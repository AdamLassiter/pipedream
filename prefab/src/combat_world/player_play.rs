use std::iter::repeat_n;

use log::debug;
use pipedream_engine::{choice::Card, description::Description, state_machine::StateMachine};

use crate::combat_world::{HUMAN_DAMAGE, HUMAN_PLAY};
use pipedream_domain::{card::PlacedCard, field::FieldPlace, player::Player, target::Target};
use pipedream_engine::{
    action::Action,
    choice::Choice,
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
    tag::Tag,
};

pub fn player_play(player: &Player, machine: &StateMachine) -> State {
    let player_hand = PlacedCard::get_placed_cards(&machine.conn, player, &FieldPlace::Hand);
    debug!(target:"Prefab/Combat/Hand", "{:?}", player_hand);

    State {
        location: HUMAN_PLAY.clone(),
        scene: Scene {
            descriptions: vec![Description::always("Play")],
        },
        choices: player_hand
            .into_iter()
            .flat_map(|(_id, PlacedCard { card: card_id, .. })| {
                Card::get_card(&machine.conn, &card_id).into_iter()
            })
            .flat_map(
                |card| {
                    let selectable = machine.tag_engine.satisfies(card.predicate);
                    let choice = Choice {
                        card,
                        selectable,
                    };
                    repeat_n(
                        choice,
                        value
                            .number()
                            .expect("Failed to get Number of cards in hand")
                            .to_num(),
                    )
                },
            )
            .collect::<Vec<_>>()
            .into(),
    }
}

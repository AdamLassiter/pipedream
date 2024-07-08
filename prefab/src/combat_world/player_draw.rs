use std::time::Duration;

use log::debug;
use rand::prelude::SliceRandom;

use crate::combat_world::{PLAYER_DRAW, PLAYER_PLAY};
use pipedream_engine::{
    combat::{entity::Ent, target::Tgt},
    core::{
        action::Action,
        scene::Scene,
        state::State,
        tags::{Tag, TagKey, TagValue},
        transition::{Transition, TransitionType},
    },
    state::combat_state_machine::CombatStateMachine,
};

pub fn player_draw(machine: &CombatStateMachine) -> State {
    let player_draw_count = machine
        .tag_engine
        .find(&Tgt::Me.ent(Ent::DrawCount))
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

    let mut player_deck_slice = machine.tag_engine.find(&Tgt::Me.ent(Ent::Deck));
    player_deck_slice.shuffle(&mut rand::thread_rng());
    let player_draw_cards = player_deck_slice
        .into_iter()
        .take(player_draw_count.to_num())
        .collect::<Vec<_>>();
    debug!(target:"Combat/Draw", "{:?}", player_draw_cards);

    let player_hand_cards = player_draw_cards
        .iter()
        .map(|player_draw_card| Tag {
            value: player_draw_card.value.clone(),
            key: TagKey(player_draw_card.key.0.replace(
                String::from(Ent::Deck).as_str(),
                String::from(Ent::Hand).as_str(),
            )),
        })
        .collect::<Vec<_>>();

    State {
        location: PLAYER_DRAW.clone(),
        scene: Scene {
            descriptions: vec!["Draw!".into()],
        },
        options: (
            Transition {
                next: TransitionType::Goto(PLAYER_PLAY.clone()),
                actions: player_draw_cards
                    .into_iter()
                    .map(|draw: Tag| Action::Subtract(draw.key.0.into()))
                    .chain(
                        player_hand_cards
                            .into_iter()
                            .map(|hand: Tag| Action::Add(hand.key.0.into())),
                    )
                    .collect(),
            },
            Duration::from_secs(2),
        )
            .into(),
    }
}

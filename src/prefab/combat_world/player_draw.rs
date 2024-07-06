use std::time::Duration;

use log::debug;
use rand::prelude::SliceRandom;

use crate::{
    engine::{
        core::{
            action::Action,
            scene::Scene,
            state::State,
            tag::{Tag, TagKey, TagValue, ME_REF, YOU_REF},
            transition::{Transition, TransitionType},
        },
        state::{combat_state_machine::*, combat_world::*},
    },
    prefab::combat_world::{PLAYER_DRAW, PLAYER_PLAY},
};

impl CombatWorld {
    pub fn player_draw_phase(machine: &CombatStateMachine) -> State {
        let player_draw_count = machine
            .tag_engine
            .find(&MY_DRAW_COUNT)
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
        let mut player_deck_slice = machine.tag_engine.find(&MY_DECK);
        player_deck_slice.shuffle(&mut rand::thread_rng());
        let player_draw_cards = player_deck_slice
            .into_iter()
            .take(player_draw_count.to_num())
            .collect::<Vec<_>>();
        debug!(target:"Combat/Draw", "{:?}", player_draw_cards);

        let me = match machine.tag_engine.tags.get(&ME_REF) {
            Some(TagValue::Tag(me)) => me,
            _ => panic!("Failed to resolve $my reference in combat"),
        };
        let you = match machine.tag_engine.tags.get(&YOU_REF) {
            Some(TagValue::Tag(you)) => you,
            _ => panic!("Failed to resolve $your reference in combat"),
        };

        let player_hand_cards = player_draw_cards
            .iter()
            .map(|player_draw_card| Tag {
                value: player_draw_card.value.clone(),
                key: TagKey(
                    player_draw_card
                        .key
                        .resolve(me, you)
                        .0
                        .replace(&PLAYER_DECK.0, &PLAYER_HAND.0),
                ),
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
}

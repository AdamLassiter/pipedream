use std::collections::BTreeMap;

use rand::prelude::SliceRandom;

use crate::{
    engine::tag_engine::TagEngine,
    resource::{
        action::Action,
        scene::Scene,
        state::State,
        tag::Tag,
        transition::{Transition, TransitionType},
        world::combat_world::{CombatWorld, DynamicStateFn},
    },
};

use super::tags::*;

impl CombatWorld {
    pub fn generate() -> Self {
        let states = {
            let mut states = BTreeMap::new();
            states.insert(
                COMBAT_INIT.clone(),
                DynamicStateFn::new(Self::combat_init_phase),
            );
            states.insert(
                PLAYER_DRAW.clone(),
                DynamicStateFn::new(Self::player_draw_phase),
            );
            states.insert(
                PLAYER_PLAY.clone(),
                DynamicStateFn::new(Self::player_play_phase),
            );
            states
        };

        CombatWorld { states }
    }

    pub fn combat_init_phase(tag_engine: &TagEngine) -> State {
        let enemy_name_slice = tag_engine.find(&ENEMY_NAME);
        let Tag(challenger, _) = enemy_name_slice.first().unwrap();
        State {
            location: COMBAT_INIT.clone(),
            scene: Scene {
                descriptions: vec![
                    "A challenger appears!".into(),
                    format!("{:?} is looking for a fight", challenger.0).into(),
                ],
            },
            options: Transition {
                next: TransitionType::Goto(PLAYER_DRAW.clone()),
                actions: vec![],
            }
            .into(),
        }
    }

    pub fn player_draw_phase(tag_engine: &TagEngine) -> State {
        let player_deck_slice = tag_engine.find(&PLAYER_DECK);
        let player_draw_card = player_deck_slice
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        let player_hand_card = player_draw_card
            .0
            .replace(&PLAYER_DECK.0, &PLAYER_HAND.0)
            .into();

        State {
            location: PLAYER_DRAW.clone(),
            scene: Scene {
                descriptions: vec!["Draw!".into()],
            },
            options: Transition {
                next: TransitionType::Goto(PLAYER_PLAY.clone()),
                actions: vec![
                    Action::Remove(player_draw_card),
                    Action::Add(player_hand_card),
                ],
            }
            .into(),
        }
    }

    pub fn player_play_phase(tag_engine: &TagEngine) -> State {
        let player_hand_slice = tag_engine.find(&PLAYER_HAND);
        State {
            location: PLAYER_PLAY.clone(),
            scene: Scene {
                descriptions: vec![],
            },

            options: player_hand_slice
                .iter()
                .map(|Tag(card, _)| {
                    (
                        format!("Play {:?}", card.0).into(),
                        Transition {
                            next: TransitionType::Goto(PLAYER_RESOLVE_PLAY.clone()),
                            actions: vec![],
                        },
                    )
                })
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

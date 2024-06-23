use std::{collections::BTreeMap, fs::File};

use rand::prelude::SliceRandom;

use crate::{
    engine::{state_machine::combat_state_machine::*, tag_engine::TagEngine},
    resource::{
        combat::card::Cards,
        core::{
            action::Action,
            scene::Scene,
            state::State,
            tag::{Tag, TagKey},
            transition::{Transition, TransitionType},
        },
        world::combat_world::{CombatWorld, DynamicStateFn},
    },
};

use super::{npcs::ENEMY_NAME, tags::Static};

pub static PLAYER_HAND: Static<TagKey> = Static::new(|| TagKey("player:hand".to_string()));
pub static PLAYER_DECK: Static<TagKey> = Static::new(|| TagKey("player:deck".to_string()));
pub static ENEMY_HAND: Static<TagKey> = Static::new(|| TagKey("enemy:hand".to_string()));
pub static ENEMY_DECK: Static<TagKey> = Static::new(|| TagKey("enemy:deck".to_string()));

impl CombatWorld {
    fn dump(&self) {
        let buffer = File::create("./combat-world.yml").unwrap();
        serde_yml::to_writer(buffer, &self).unwrap();
    }

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

        let world = CombatWorld {
            states,
            cards: Cards::generate(),
        };

        world.dump();
        world
    }

    pub fn combat_init_phase(_machine: &CombatStateMachine, tag_engine: &TagEngine) -> State {
        let enemy_name_slice = tag_engine.find(&ENEMY_NAME);
        let Tag {
            key: challenger, ..
        } = enemy_name_slice.first().unwrap();
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

    pub fn player_draw_phase(_machine: &CombatStateMachine, tag_engine: &TagEngine) -> State {
        let player_deck_slice = tag_engine.find(&PLAYER_DECK);
        let player_draw_card = player_deck_slice
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();

        let player_hand_card = player_draw_card
            .key
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

    pub fn player_play_phase(machine: &CombatStateMachine, tag_engine: &TagEngine) -> State {
        let player_hand_slice = tag_engine.find(&PLAYER_HAND);
        State {
            location: PLAYER_PLAY.clone(),
            scene: Scene {
                descriptions: vec![],
            },

            options: player_hand_slice
                .iter()
                .map(|Tag { key: card, .. }| machine.combat_world.cards.find(card))
                .filter(|&card_data| tag_engine.satisfies(&card_data.predicate))
                .map(|card_data| {
                    (
                        format!("Play {:?} {:?}", card_data.name, card_data.predicate).into(),
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

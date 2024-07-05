use std::{collections::BTreeMap, time::Duration};

use log::debug;
use rand::prelude::SliceRandom;

use crate::engine::{
    combat::{
        card::Cards,
        npc::{Npcs, ENEMY_NAME},
    },
    core::{
        action::Action,
        scene::Scene,
        state::State,
        tag::{Tag, TagKey, TagValue, ME_REF, YOU_REF},
        transition::{Transition, TransitionType},
    },
    state::{combat_state_machine::*, combat_world::*},
};

impl CombatWorld {
    pub fn generate() -> Self {
        let states = {
            BTreeMap::from_iter(vec![
                (
                    COMBAT_INIT.clone(),
                    DynamicStateFn::new(Self::combat_init_phase),
                ),
                (
                    PLAYER_DRAW.clone(),
                    DynamicStateFn::new(Self::player_draw_phase),
                ),
                (
                    PLAYER_PLAY.clone(),
                    DynamicStateFn::new(Self::player_play_phase),
                ),
                (
                    PLAYER_RESOLVE_PLAY.clone(),
                    DynamicStateFn::new(Self::player_resolve_phase),
                ),
            ])
        };

        Self {
            states,
            cards: Cards::generate(),
            npcs: Npcs::generate(),
        }
    }

    fn combat_init_phase(machine: &CombatStateMachine) -> State {
        let enemy_name_slice = machine.tag_engine.find(&ENEMY_NAME);
        let Tag { key: enemy, .. } = enemy_name_slice
            .first()
            .expect("Failed to find enemy name slice");
        let enemy_data = machine.combat_world.npcs.find(enemy);

        State {
            location: COMBAT_INIT.clone(),
            scene: Scene {
                descriptions: vec![
                    "A challenger appears!".into(),
                    format!("{:?} is looking for a fight", enemy_data.name).into(),
                ],
            },
            options: (
                Transition {
                    next: TransitionType::Goto(PLAYER_DRAW.clone()),
                    actions: vec![],
                },
                Duration::from_secs(2),
            )
                .into(),
        }
    }

    fn player_draw_phase(machine: &CombatStateMachine) -> State {
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

        let me = match machine.tag_engine.tags.get(&ME_REF) {
            Some(TagValue::Tag(me)) => me,
            _ => panic!("Failed to resolve :me reference in combat"),
        };
        let you = match machine.tag_engine.tags.get(&YOU_REF) {
            Some(TagValue::Tag(you)) => you,
            _ => panic!("Failed to resolve :you reference in combat"),
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
        debug!(target:"Combat/Draw", "{:?}", player_hand_cards);

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

    fn player_play_phase(machine: &CombatStateMachine) -> State {
        let player_hand_slice = machine.tag_engine.find(&MY_HAND);
        debug!(target:"Combat/Hand", "{:?}", player_hand_slice);

        State {
            location: PLAYER_PLAY.clone(),
            scene: Scene {
                descriptions: vec!["Play".into()],
            },
            options: player_hand_slice
                .iter()
                .map(|Tag { key: card, .. }| machine.combat_world.cards.find(card))
                .map(|card_data| {
                    (
                        format!("Play {:?} [{}]", card_data.name, card_data.predicate).into(),
                        Transition {
                            next: TransitionType::Goto(PLAYER_RESOLVE_PLAY.clone()),
                            actions: card_data.actions.clone(),
                        },
                        machine.tag_engine.satisfies(&card_data.predicate),
                    )
                })
                .collect::<Vec<_>>()
                .into(),
        }
    }

    fn player_resolve_phase(_machine: &CombatStateMachine) -> State {
        State {
            location: PLAYER_RESOLVE_PLAY.clone(),
            scene: Scene {
                descriptions: vec![],
            },
            options: Transition {
                next: TransitionType::Goto(ENEMY_DRAW.clone()),
                actions: vec![],
            }
            .into(),
        }
    }
}

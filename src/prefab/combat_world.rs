use std::collections::BTreeMap;

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
        tag::Tag,
        transition::{Transition, TransitionType},
    },
    state::{combat_state_machine::*, combat_world::*},
};

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

        Self {
            states,
            cards: Cards::generate(),
            npcs: Npcs::generate(),
        }
    }

    pub fn combat_init_phase(machine: &CombatStateMachine) -> State {
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
            options: Transition {
                next: TransitionType::Goto(PLAYER_DRAW.clone()),
                actions: vec![],
            }
            .into(),
        }
    }

    pub fn player_draw_phase(machine: &CombatStateMachine) -> State {
        let player_deck_slice = machine.tag_engine.find(&PLAYER_DECK);
        let player_draw_card = player_deck_slice
            .choose(&mut rand::thread_rng())
            .expect("Failed to generate thread RNG")
            .clone();

        let player_hand_card = player_draw_card
            .key
            .replace(&PLAYER_DECK.0, &PLAYER_HAND.0)
            .into();
        debug!(target:"Combat/Draw", "{:?}", player_draw_card);

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

    pub fn player_play_phase(machine: &CombatStateMachine) -> State {
        let player_hand_slice = machine.tag_engine.find(&PLAYER_HAND);
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
                            actions: vec![],
                        },
                        machine.tag_engine.satisfies(&card_data.predicate),
                    )
                })
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

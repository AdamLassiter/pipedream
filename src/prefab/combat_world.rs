use std::collections::BTreeMap;

use crate::{
    engine::tag_engine::TagEngine,
    resource::{
        location::Location,
        scene::Scene,
        state::State,
        tag::Tag,
        transition::{Transition, TransitionType},
        world::combat_world::{CombatWorld, DynamicStateFn},
    },
};

use super::tags::{COMBAT_INIT, ENEMY_NAME, PLAYER_CARD, PLAYER_DRAW, PLAYER_PLAY};

impl CombatWorld {
    pub fn generate() -> Self {
        let states = {
            let mut states = BTreeMap::new();
            states.insert(
                Location(COMBAT_INIT.into()),
                DynamicStateFn::new(Self::combat_init_phase),
            );
            states.insert(
                Location(PLAYER_DRAW.into()),
                DynamicStateFn::new(Self::player_draw_phase),
            );
            states.insert(
                Location(PLAYER_PLAY.into()),
                DynamicStateFn::new(Self::player_play_phase),
            );
            states
        };

        CombatWorld { states }
    }

    pub fn player_play_phase(_tag_engine: &TagEngine) -> State {
        State {
            location: Location(PLAYER_PLAY.into()),
            scene: Scene {
                descriptions: vec![],
            },
            options: vec![].into(),
        }
    }

    pub fn player_draw_phase(tag_engine: &TagEngine) -> State {
        let player_cards_slice = tag_engine.find(&PLAYER_CARD.into());
        State {
            location: Location(PLAYER_DRAW.into()),
            scene: Scene {
                descriptions: player_cards_slice
                    .iter()
                    .map(|Tag(card, _)| format!("Play {:?}", card.0).into())
                    .collect(),
            },
            options: vec![].into(),
        }
    }

    pub fn combat_init_phase(tag_engine: &TagEngine) -> State {
        let enemy_name_slice = tag_engine.find(&ENEMY_NAME.into());
        let Tag(challenger, _) = enemy_name_slice.first().unwrap();
        State {
            location: Location(COMBAT_INIT.into()),
            scene: Scene {
                descriptions: vec![
                    "A challenger appears!".into(),
                    format!("{:?} is looking for a fight", challenger.0).into(),
                ],
            },
            options: Transition {
                next: TransitionType::Goto(Location(PLAYER_DRAW.into())),
                actions: vec![],
            }
            .into(),
        }
    }
}

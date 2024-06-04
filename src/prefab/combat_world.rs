use std::collections::BTreeMap;

use crate::{
    engine::tag_engine::TagEngine,
    resource::{
        location::Location,
        scene::Scene,
        state::State,
        tag::Tag,
        world::combat_world::{DynamicStateFn, DynamicWorld},
    },
};

pub struct CombatWorld {
    states: BTreeMap<Location, DynamicStateFn>,
}

impl DynamicWorld for CombatWorld {
    fn get_state(&self, location: &Location) -> &DynamicStateFn {
        self.states.get(location).unwrap()
    }
}

impl CombatWorld {
    pub fn generate() -> Self {
        let states = {
            let mut states = BTreeMap::new();
            states.insert(
                Location("player:draw".into()),
                DynamicStateFn::new(Self::player_draw_phase),
            );
            states.insert(
                Location("player:play".into()),
                DynamicStateFn::new(Self::player_play_phase),
            );
            states
        };

        CombatWorld { states }
    }

    pub fn player_play_phase(_tag_engine: &TagEngine) -> State {
        State {
            location: Location("player:play".into()),
            scene: Scene {
                descriptions: vec![],
            },
            options: vec![].into(),
        }
    }

    pub fn player_draw_phase(tag_engine: &TagEngine) -> State {
        let enemy_name_slice = tag_engine.find(&"enemy:name".into());
        let Tag(challenger, _) = enemy_name_slice.first().unwrap();
        State {
            location: Location("player:draw".into()),
            scene: Scene {
                descriptions: vec![
                    "A challenger appears!".into(),
                    format!("{:?} is looking for a fight", challenger)
                        .to_string()
                        .as_str()
                        .into(),
                ],
            },
            options: vec![].into(),
        }
    }
}

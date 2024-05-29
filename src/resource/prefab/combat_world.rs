use std::collections::BTreeMap;

use crate::{engine::dynamic_state_machine::DynamicStateMachine, resource::state::State};

use crate::resource::{
    action::Action,
    dynamic_world::{DynamicStateFn, DynamicWorld},
    location::Location,
    predicate::Predicate,
    scene::Scene,
    transition::{SideEffect, TransitionType},
};

pub struct CombatWorld {
    states: BTreeMap<Location, DynamicStateFn<Self>>,
}

impl DynamicWorld for CombatWorld {
    fn get_state(&self, location: &Location) -> &DynamicStateFn<Self> {
        self.states.get(location).unwrap()
    }
}

impl CombatWorld {
    pub fn generate_combat() -> Self {
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

    pub fn player_play_phase(_state_machine: &DynamicStateMachine<Self>) -> State {
        State {
            location: Location("player:play".into()),
            scene: Scene {
                descriptions: vec![
                    "You are in the woods".into(),
                    "There is a mysterious moss-covered shop in a small grove".into(),
                    (
                        Predicate::Tag("woods:entrance:item:sword".into()),
                        "You see a shiny sword lodged in a stone".into(),
                    )
                        .into(),
                ],
            },
            options: vec![
                (
                    (
                        Predicate::Tag("woods:entrance:item:sword".into()),
                        "Pick up the sword".into(),
                    )
                        .into(),
                    SideEffect {
                        next: TransitionType::None,
                        actions: vec![
                            Action::Insert("player:item:sword".into()),
                            Action::Remove("woods:entrance:item:sword".into()),
                        ],
                    },
                ),
                (
                    "Go into the shop".into(),
                    SideEffect {
                        next: TransitionType::Push(Location("ephemeral:shop".into())),
                        actions: vec![],
                    },
                ),
                (
                    "Go deeper into the woods".into(),
                    SideEffect {
                        next: TransitionType::Swap(Location("woods:depths".into())),
                        actions: vec![],
                    },
                ),
            ]
            .into(),
        }
    }

    pub fn player_draw_phase(_state_machine: &DynamicStateMachine<Self>) -> State {
        State {
            location: Location("player:draw".into()),
            scene: Scene {
                descriptions: vec!["A challenger appears".into()],
            },
            options: vec![].into(),
        }
    }
}

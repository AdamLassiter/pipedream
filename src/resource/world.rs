use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    resource::action::Action,
    resource::scene::Scene,
    resource::{state::State, transition::SideEffect},
};

use super::{location::Location, predicate::Predicate, transition::TransitionType};

pub trait World {
    fn get_state(&self, location: &Location) -> &State;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignWorld(BTreeMap<Location, State>);

impl World for CampaignWorld {
    fn get_state(&self, location: &Location) -> &State {
        self.0.get(location).unwrap()
    }
}

impl CampaignWorld {
    pub fn generate_campaign() -> CampaignWorld {
        CampaignWorld(
            vec![
                State {
                    location: Location("woods:entrance".into()),
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
                .into(),
                State {
                    location: Location("woods:depths".into()),
                    scene: Scene {
                        descriptions: vec!["You are lost in the woods".into()],
                    },
                    options: vec![(
                        "Go deeper into the woods".into(),
                        SideEffect {
                            next: TransitionType::Swap(Location("woods:depths".into())),
                            actions: vec![],
                        },
                    )]
                    .into(),
                }
                .into(),
                State {
                    location: Location("ephemeral:shop".into()),
                    scene: Scene {
                        descriptions: vec![
                            "The shop is cozy, and staffed by a weathered crone".into(),
                            (
                                Predicate::Tag("player:item:sword".into()),
                                "Her eyes keep flitting to the sword at your side".into(),
                            )
                                .into(),
                        ],
                    },
                    options: vec![
                        (
                            "Leave the shop".into(),
                            SideEffect {
                                next: TransitionType::Pop,
                                actions: vec![],
                            },
                        ),
                        (
                            (
                                Predicate::Tag("player:item:sword".into()),
                                "Trade a sword for two swords".into(),
                            )
                                .into(),
                            SideEffect {
                                next: TransitionType::None,
                                actions: vec![
                                    Action::Subtract("player:item:sword/1".into()),
                                    Action::Add("player:item:sword/2".into()),
                                ],
                            },
                        ),
                        (
                            (
                                Predicate::Tag("player:item:sword".into()),
                                "Trade each sword for two swords".into(),
                            )
                                .into(),
                            SideEffect {
                                next: TransitionType::None,
                                actions: vec![Action::Multiply("player:item:sword/2".into())],
                            },
                        ),
                        (
                            (
                                Predicate::Tag("player:item:sword/2".into()),
                                "Forge two swords into a cursed ring".into(),
                            )
                                .into(),
                            SideEffect {
                                next: TransitionType::None,
                                actions: vec![
                                    Action::Subtract("player:item:sword/2".into()),
                                    Action::Add("player:item:cursed-ring".into()),
                                ],
                            },
                        ),
                        (
                            (
                                Predicate::Tag("player:item:sword/2".into()),
                                "Forge every other sword into a cursed ring".into(),
                            )
                                .into(),
                            SideEffect {
                                next: TransitionType::None,
                                actions: vec![
                                    Action::Divide("player:item:sword/2".into()),
                                    Action::Add("player:item:cursed-ring/player:item:sword".into()),
                                ],
                            },
                        ),
                    ]
                    .into(),
                }
                .into(),
            ]
            .into_iter()
            .collect(),
        )
    }
}

#[derive(Debug)]
pub struct CombatWorld<CombatState> {
    states: BTreeMap<Location, fn(CombatState) -> State>,
}

impl World for CombatWorld<()> {
    fn get_state(&self, location: &Location) -> &State {
        todo!()
    }
}

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    resource::action::Action,
    resource::scene::Scene,
    resource::{state::State, transition::SideEffect},
};

use super::{location::Location, predicate::Predicate, transition::TransitionType};

#[derive(Debug, Serialize, Deserialize)]
pub struct World(pub BTreeMap<Location, State>);

impl World {
    pub fn generate() -> World {
        World(
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
                                "Give her the sword".into(),
                            )
                                .into(),
                            SideEffect {
                                next: TransitionType::None,
                                actions: vec![
                                    Action::Remove("player:item:sword".into()),
                                    Action::Insert("player:item:cursed-ring".into()),
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

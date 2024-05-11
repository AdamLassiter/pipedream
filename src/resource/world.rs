use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    resource::action::Action,
    resource::scene::Scene,
    resource::{state::State, transition::Transition},
};

use super::{location::Location, predicate::Predicate};

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
                            (
                                Predicate::Tag("woods:entrance:item:sword".into()),
                                "You see a shiny sword lodged in a stone".into(),
                            )
                                .into(),
                        ],
                    },
                    options: vec![
                        (
                            "Go deeper into the woods".into(),
                            Transition {
                                next: Location("woods:depths".into()),
                                actions: vec![],
                            },
                        ),
                        (
                            (
                                Predicate::Tag("woods:entrance:item:sword".into()),
                                "Pick up the sword".into(),
                            )
                                .into(),
                            Transition {
                                next: Location("woods:entrance".into()),
                                actions: vec![
                                    Action::Insert("player:item:sword".into()),
                                    Action::Remove("woods:entrance:item:sword".into()),
                                ],
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
                        Transition {
                            next: Location("woods:depths".into()),
                            actions: vec![],
                        },
                    )]
                    .into(),
                }
                .into(),
            ]
            .into_iter()
            .collect(),
        )
    }
}

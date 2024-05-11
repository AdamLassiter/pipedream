use std::{collections::BTreeMap, fs::File};

use serde_derive::Serialize;

use crate::{
    resource::scene::Scene,
    resource::{action::Action, tag::Tag},
    resource::{state::State, transition::Transition},
};

use super::location::Location;

#[derive(Debug, Serialize)]
pub struct World(pub BTreeMap<Location, State>);

impl World {
    pub fn generate() -> World {
        World(
            vec![
                State {
                    location: Location("woods:entrance".into()),
                    scene: Scene {
                        descriptions: vec!["You are in the woods".into()],
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
                            "Pick up the sword".into(),
                            Transition {
                                next: Location("woods:entrance".into()),
                                actions: vec![
                                    Action::Insert(Tag("player:item:sword".into())),
                                    Action::Insert(Tag("woods:entrance:item:no-sword".into())),
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

    pub fn dump(&self) {
        let buffer = File::create("./world.yaml").unwrap();
        serde_yaml::to_writer(buffer, &self).unwrap();
    }
}

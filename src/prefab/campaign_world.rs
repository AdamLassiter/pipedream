use std::fs::File;

use crate::resource::core::{
    action::Action,
    location::Location,
    predicate::Predicate,
    scene::Scene,
    state::State,
    transition::{Transition, TransitionType},
};
use crate::resource::world::campaign_world::CampaignWorld;

impl CampaignWorld {
    fn dump(&self) {
        let buffer = File::create("./campaign-world.yml")
            .expect("Failed to open file for writing campaign-world data");
        serde_yml::to_writer(buffer, &self)
            .expect("Failed to write yaml campaign-world data to file");
    }

    pub fn generate() -> Self {
        let world = CampaignWorld {
            states: vec![
                State {
                    location: Location("woods:entrance".into()),
                    scene: Scene {
                        descriptions: vec![
                            "You are in the woods".into(),
                            "There is a mysterious moss-covered shop in a small grove".into(),
                            (
                                Predicate::Tag("woods:entrance:item:sword".into()),
                                "You see a shiny sword lodged in a stone",
                            )
                                .into(),
                        ],
                    },
                    options: vec![
                        (
                            (
                                Predicate::Tag("woods:entrance:item:sword".into()),
                                "Pick up the sword",
                            )
                                .into(),
                            Transition {
                                next: TransitionType::None,
                                actions: vec![
                                    Action::Insert("player:item:sword".into()),
                                    Action::Remove("woods:entrance:item:sword".into()),
                                ],
                            },
                        ),
                        (
                            "Go into the shop".into(),
                            Transition {
                                next: TransitionType::Enter(Location("ephemeral:shop".into())),
                                actions: vec![],
                            },
                        ),
                        (
                            "Go deeper into the woods".into(),
                            Transition {
                                next: TransitionType::Goto(Location("woods:depths".into())),
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
                    options: vec![
                        (
                            "Go deeper into the woods".into(),
                            Transition {
                                next: TransitionType::Goto(Location("woods:depths".into())),
                                actions: vec![],
                            },
                        ),
                        (
                            "Battle inner demons".into(),
                            Transition {
                                next: TransitionType::Combat(vec![
                                    Action::Add("enemy:name:Dave".into()),
                                    Action::Add("player:deck:Anathema Device".into()),
                                    Action::Add("player:deck:Bag of Endless Bags".into()),
                                ]),
                                actions: vec![],
                            },
                        ),
                    ]
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
                                "Her eyes keep flitting to the sword at your side",
                            )
                                .into(),
                        ],
                    },
                    options: vec![
                        (
                            "Leave the shop".into(),
                            Transition {
                                next: TransitionType::Leave,
                                actions: vec![],
                            },
                        ),
                        (
                            (
                                Predicate::Tag("player:item:sword".into()),
                                "Trade a sword for two swords",
                            )
                                .into(),
                            Transition {
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
                                "Trade each sword for two swords",
                            )
                                .into(),
                            Transition {
                                next: TransitionType::None,
                                actions: vec![Action::Multiply("player:item:sword/2".into())],
                            },
                        ),
                        (
                            (
                                Predicate::Tag("player:item:sword/2".into()),
                                "Forge two swords into a cursed ring",
                            )
                                .into(),
                            Transition {
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
                                "Forge every other sword into a cursed ring",
                            )
                                .into(),
                            Transition {
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
        };

        world.dump();
        world
    }
}

use crate::resource::state::State;

use crate::resource::{
    action::Action,
    location::Location,
    predicate::Predicate,
    scene::Scene,
    transition::{Transition, TransitionType},
    world::campaign_world::CampaignWorld,
};

impl CampaignWorld {
    pub fn generate() -> CampaignWorld {
        CampaignWorld {
            states: vec![
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
                                next: TransitionType::Combat(vec![Action::Add(
                                    "enemy:name:Dave".into(),
                                )]),
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
                                "Her eyes keep flitting to the sword at your side".into(),
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
                                "Trade a sword for two swords".into(),
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
                                "Trade each sword for two swords".into(),
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
                                "Forge two swords into a cursed ring".into(),
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
                                "Forge every other sword into a cursed ring".into(),
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
        }
    }
}

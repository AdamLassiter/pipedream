use crate::Generatable;
use pipedream_engine::{
    core::{
        action::Action,
        choice::Choice,
        location::Location,
        predicate::Predicate,
        scene::Scene,
        state::State,
        tags::Static,
        transition::{Transition, TransitionType},
    },
    state::campaign_world::CampaignWorld,
};

pub static CAMPAIGN_DEFEAT: Static<Location> = Static::new(|| "campaign:defeat".into());

impl Generatable for CampaignWorld {
    fn generate() -> Self {
        let vec = vec![
            State {
                location: "woods:entrance".into(),
                scene: Scene {
                    descriptions: vec![
                        "You are in <green the woods>".into(),
                        "There is a mysterious moss-covered shop in a small grove".into(),
                        (
                            Predicate::Tag("woods:entrance:item:sword".into()),
                            "You see a shiny sword lodged in a stone",
                        )
                            .into(),
                    ],
                },
                options: vec![
                    Choice {
                        summary:
                            "Pick up the sword"
                            .into(),
                        predicate: Some(Predicate::Tag("woods:entrance:item:sword".into())),
                        effect: Transition {
                            next: TransitionType::None,
                            actions: vec![
                                Action::Insert("player:item:sword".into()),
                                Action::Remove("woods:entrance:item:sword".into()),
                            ],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Go into the shop".into(),
                        image: Some("resources/objects/glade-objects-top-down-pixel-art/png/objects_separated/assets_no_shadow/house1.png".into()),
                        effect: Transition {
                            next: TransitionType::Enter("ephemeral:shop".into()),
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Go deeper into the woods".into(),
                        image: Some("resources/objects/forest-objects-top-down-pixel-art/png/assets/luminous_tree2.png".into()),
                        effect: Transition {
                            next: TransitionType::Goto("woods:depths".into()),
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                ]
                .into(),
            },
            State {
                location: "woods:depths".into(),
                scene: Scene {
                    descriptions: vec!["You are lost in <green the woods>".into()],
                },
                options: vec![
                    Choice { 
                        summary: "Go deeper into the woods".into(),
                        image: Some("resources/objects/forest-objects-top-down-pixel-art/png/assets/luminous_tree2.png".into()),
                        effect: Transition {
                            next: TransitionType::Goto("woods:depths".into()),
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                    Choice { 
                        summary: "Battle <red inner demons>".into(),
                        image: Some("resources/rpg/demon-avatar-icons-pixel-art-64x64/png/transperent/icon42.png".into()),
                        effect: Transition {
                            next: TransitionType::Combat(vec![Action::Add(
                                "enemy:name:Dave".into(),
                            )]),
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                ]
                .into(),
            },
            State {
                location: "ephemeral:shop".into(),
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
                    Choice {
                        summary: "Leave the shop".into(),
                        image: Some("resources/profiles/dark-elf-characters-full-length-pixel-art/png/dark elves_faces_transperent/character6_face1.png".into()),
                        effect: Transition {
                            next: TransitionType::Leave,
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Trade a sword for two swords".into(),
                        image: Some("resources/profiles/dark-elf-characters-full-length-pixel-art/png/dark elves_faces_transperent/character6_face2.png".into()),
                        predicate: Some(Predicate::Tag("player:item:sword".into())),
                        effect: Transition {
                            next: TransitionType::None,
                            actions: vec![
                                Action::Subtract("player:item:sword/1".into()),
                                Action::Add("player:item:sword/2".into()),
                            ],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Trade every sword for two swords".into(),
                        image: Some("resources/profiles/dark-elf-characters-full-length-pixel-art/png/dark elves_faces_transperent/character6_face2.png".into()),
                        predicate: Some(Predicate::Tag("player:item:sword".into())),
                        effect: Transition {
                            next: TransitionType::None,
                            actions: vec![Action::Multiply("player:item:sword/2".into())],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Forge a pair of swords into a cursed ring".into(),
                        image: Some("resources/profiles/dark-elf-characters-full-length-pixel-art/png/dark elves_faces_transperent/character6_face3.png".into()),
                        predicate: Some(Predicate::Tag("player:item:sword/2".into())),
                        effect: Transition {
                            next: TransitionType::None,
                            actions: vec![
                                Action::Subtract("player:item:sword/2".into()),
                                Action::Add("player:item:cursed-ring".into()),
                            ],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Forge every pair of swords into a cursed ring".into(),
                        image: Some("resources/profiles/dark-elf-characters-full-length-pixel-art/png/dark elves_faces_transperent/character6_face4.png".into()),
                        predicate: Some(Predicate::Tag("player:item:sword/2".into())),
                        effect: Transition {
                            next: TransitionType::None,
                            actions: vec![
                                Action::Divide("player:item:sword/2".into()),
                                Action::Add("player:item:cursed-ring/player:item:sword".into()),
                            ],
                        },
                        ..Default::default()
                    },
                ]
                .into(),
            },
        ];
        vec
        .into()
    }
}

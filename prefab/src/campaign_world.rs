use crate::Generatable;
use pipedream_engine::{
    core::{
        action::Action,
        choice::Choice,
        location::Location,
        predicate::Predicate,
        scene::Scene,
        state::State,
        effect::{Effect, Transition},
    },
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
                choices: vec![
                    Choice {
                        summary:
                            "Pick up the sword"
                            .into(),
                        image: Some("resources/hi-res/sword/png/without_shadow/7.png".into()),
                        predicate: Some(Predicate::Tag("woods:entrance:item:sword".into())),
                        effect: Effect {
                            transition: Transition::None,
                            actions: vec![
                                Action::Add("player:item:sword".into()),
                                Action::Remove("woods:entrance:item:sword".into()),
                            ],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Go into the shop".into(),
                        image: Some("resources/scenery/glade/png/objects_separated/assets_no_shadow/house1.png".into()),
                        effect: Effect {
                            transition: Transition::Enter("ephemeral:shop".into()),
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Go deeper into the woods".into(),
                        image: Some("resources/scenery/forest/png/assets_no_shadow/luminous_tree1.png".into()),
                        effect: Effect {
                            transition: Transition::Goto("woods:depths".into()),
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
                choices: vec![
                    Choice { 
                        summary: "Go deeper into the woods".into(),
                        image: Some("resources/scenery/forest/png/assets_no_shadow/luminous_tree2.png".into()),
                        effect: Effect {
                            transition: Transition::Goto("woods:depths".into()),
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                    Choice { 
                        summary: "Battle <red inner demons>".into(),
                        image: Some("resources/avatars/demon-hires/png/transperent/icon42.png".into()),
                        effect: Effect {
                            transition: Transition::Combat(vec![Action::Add(
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
                choices: vec![
                    Choice {
                        summary: "Leave the shop".into(),
                        image: Some("resources/avatars/dark-elf-2/png/dark elves_faces_transperent/character6_face1.png".into()),
                        effect: Effect {
                            transition: Transition::Leave,
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Trade a sword for two swords".into(),
                        image: Some("resources/avatars/dark-elf-2/png/dark elves_faces_transperent/character6_face2.png".into()),
                        predicate: Some(Predicate::Tag("player:item:sword".into())),
                        effect: Effect {
                            transition: Transition::None,
                            actions: vec![
                                Action::Subtract("player:item:sword=1".into()),
                                Action::Add("player:item:sword=2".into()),
                            ],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Trade every sword for two swords".into(),
                        image: Some("resources/avatars/dark-elf-2/png/dark elves_faces_transperent/character6_face2.png".into()),
                        predicate: Some(Predicate::Tag("player:item:sword".into())),
                        effect: Effect {
                            transition: Transition::None,
                            actions: vec![Action::Multiply("player:item:sword=2".into())],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Forge a pair of swords into a cursed ring".into(),
                        image: Some("resources/avatars/dark-elf-2/png/dark elves_faces_transperent/character6_face3.png".into()),
                        predicate: Some(Predicate::Tag("player:item:sword=2".into())),
                        effect: Effect {
                            transition: Transition::None,
                            actions: vec![
                                Action::Subtract("player:item:sword=2".into()),
                                Action::Add("player:item:cursed-ring".into()),
                            ],
                        },
                        ..Default::default()
                    },
                    Choice {
                        summary: "Forge every pair of swords into a cursed ring".into(),
                        image: Some("resources/avatars/dark-elf-2/png/dark elves_faces_transperent/character6_face4.png".into()),
                        predicate: Some(Predicate::Tag("player:item:sword=2".into())),
                        effect: Effect {
                            transition: Transition::None,
                            actions: vec![
                                Action::Divide("player:item:sword=2".into()),
                                Action::Add("player:item:cursed-ring=player:item:sword".into()),
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

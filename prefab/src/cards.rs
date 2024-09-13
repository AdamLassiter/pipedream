use crate::{Buildable, Generatable};
use pipedream_domain::card::{Card, Cards};
use pipedream_engine::{action::Action, predicate::Predicate, tag::Tags};

impl Generatable for Cards {
    fn generate() -> Self {
        generate_vec().into()
    }
}

fn generate_vec() -> Vec<Card> {
    vec![
        Card {
            title: "Anathema Device".into(),
            image: "resources/legacy/tile269.png".into(),
            details: vec!["Apply <blue 0.5 anathema> [Self]".into()],
            cost: "<blue 10 mana>".into(),
            predicate: Predicate::Tag("$me:resource:mana=10".into()),
            actions: vec![
                Action::Subtract("$me:resource:mana=10".into()),
                Action::Add("$me:special:anathema=0.5".into()),
            ],
            tags: Tags::build(vec!["card:type:device".into()]),
        },
        Card {
            name: "Bag of Endless Bags".into(),
            image: "resources/legacy/tile198.png".into(),
            details: vec![
                "Draw <yellow 2 from deck> [Self]".into(),
                "Discard <yellow 2 from hand> [Enemy]".into(),
            ],
            cost: "<yellow 10 faith>".into(),
            predicate: Predicate::Tag("$me:resource:faith=10".into()),
            actions: vec![
                Action::Subtract("$me:resource:faith=10".into()),
                Action::Add("$me:effect:draw=2".into()),
                Action::Add("$you:effect:discard=2".into()),
            ],
            tags: Tags::build(vec!["card:type:bag".into()]),
        },
        Card {
            name: "Regular Punch".into(),
            image: "resources/legacy/tile095.png".into(),
            details: vec!["Damage <red 2 health> [Enemy]".into()],
            cost: "<green 1 stamina>".into(),
            predicate: Predicate::Tag("$me:resource:stamina=1".into()),
            actions: vec![
                Action::Subtract("$me:resource:stamina=1".into()),
                Action::Add("$you:damage:resource:health=2".into()),
            ],
            tags: Tags::build(vec!["card:type:melee".into()]),
        },
        Card {
            name: "Immolate".into(),
            image: "resources/legacy/tile009.png".into(),
            details: vec![
                "Damage <red 100% self health> [Enemy]".into(),
                "Damage <green 100% self stamina> [Enemy]".into(),
            ],
            cost: "<red 100% health>, <green 100% stamina>".into(),
            predicate: Predicate::Tag("$me:resource:health=1".into()),
            actions: vec![
                Action::Add("$you:damage:resource:health=$me:resource:health".into()),
                Action::Add("$you:damage:resource:stamina=$me:resource:stamina".into()),
                Action::Subtract("$me:resource:health=$me:resource:health".into()),
                Action::Subtract("$me:resource:stamina=$me:resource:stamina".into()),
            ],
            tags: Tags::build(vec!["card:type:melee".into()]),
        },
    ]
}

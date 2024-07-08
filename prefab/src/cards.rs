use crate::{Buildable, Generatable};
use pipedream_engine::{
    combat::card::{Card, Cards},
    core::{action::Action, predicate::Predicate, tags::Tags},
};

impl Generatable for Cards {
    fn generate() -> Self {
        generate_vec().into()
    }
}

fn generate_vec() -> Vec<Card> {
    vec![
        Card {
            name: "Anathema Device".into(),
            predicate: Predicate::Tag("$me:resource:mana/10".into()),
            actions: vec![
                Action::Subtract("$me:resource:mana/10".into()),
                Action::Add("$me:special:anathema/0.5".into()),
            ],
            tags: Tags::build(vec!["card:type:device".into()]),
        },
        Card {
            name: "Bag of Endless Bags".into(),
            predicate: Predicate::Tag("$me:resource:faith/10".into()),
            actions: vec![
                Action::Subtract("$me:resource:faith/10".into()),
                Action::Add("$me:effect:draw/2".into()),
                Action::Add("$you:effect:discard/2".into()),
            ],
            tags: Tags::build(vec!["card:type:bag".into()]),
        },
        Card {
            name: "Regular Punch".into(),
            predicate: Predicate::Tag("$me:resource:stamina/1".into()),
            actions: vec![
                Action::Subtract("$me:resource:stamina/1".into()),
                Action::Add("$you:damage:resource:health/2".into()),
            ],
            tags: Tags::build(vec!["card:type:melee".into()]),
        },
        Card {
            name: "Consecutive Regular Punches".into(),
            predicate: Predicate::Tag("$me:resource:stamina/5".into()),
            actions: vec![
                Action::Subtract("$me:resource:stamina/5".into()),
                Action::Add("$you:damage:resource:health/12".into()),
            ],
            tags: Tags::build(vec!["card:type:melee".into()]),
        },
    ]
}

use std::collections::BTreeMap;

use crate::engine::{
    combat::card::{Card, Cards},
    core::{action::Action, predicate::Predicate},
};

impl Cards {
    pub fn generate() -> Self {
        Self(BTreeMap::from_iter(
            Self::generate_vec()
                .into_iter()
                .map(|card| (card.name.clone(), card)),
        ))
    }

    fn generate_vec() -> Vec<Card> {
        vec![
            Card {
                name: "Anathema Device".into(),
                predicate: Predicate::Tag("$my:resource:mana/10".into()),
                actions: vec![Action::Add("$my:special:anathema/0.5".into())],
                tags: vec!["card:type:device".into()].into(),
            },
            Card {
                name: "Bag of Endless Bags".into(),
                predicate: Predicate::Tag("$my:resource:faith/10".into()),
                actions: vec![
                    Action::Add("$my:effect:draw/2".into()),
                    Action::Add("$your:effect:discard/2".into()),
                ],
                tags: vec!["card:type:bag".into()].into(),
            },
            Card {
                name: "Regular Punch".into(),
                predicate: Predicate::Tag("$my:resource:stamina/1".into()),
                actions: vec![Action::Subtract("$your:resource:health/2".into())],
                tags: vec!["card:type:melee".into()].into(),
            },
            Card {
                name: "Consecutive Regular Punches".into(),
                predicate: Predicate::Tag("$my:resource:stamina/5".into()),
                actions: vec![Action::Subtract("$your:resource:health/12".into())],
                tags: vec!["card:type:melee".into()].into(),
            },
        ]
    }
}

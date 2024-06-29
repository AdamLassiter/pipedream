use std::{collections::BTreeMap, fs::File};

use crate::resource::{
    combat::card::{Card, Cards},
    core::predicate::Predicate,
};

impl Cards {
    fn dump(&self) {
        let buffer =
            File::create("./cards.yml").expect("Failed to open file for writing cards data");
        serde_yml::to_writer(buffer, &self).expect("Failed to write yaml cards data to file");
    }

    pub fn generate() -> Self {
        let cards = Self(BTreeMap::from_iter(
            Self::generate_vec()
                .into_iter()
                .map(|card| (card.name.clone(), card)),
        ));

        cards.dump();
        cards
    }

    fn generate_vec() -> Vec<Card> {
        vec![
            Card {
                name: "Anathema Device".into(),
                predicate: Predicate::Tag("$my:resource:mana/10".into()),
                applies_tags: vec!["$my:special:anathema/0.5".into()].into(),
                has_tags: vec!["card:type:device".into()].into(),
            },
            Card {
                name: "Bag of Endless Bags".into(),
                predicate: Predicate::Tag("$my:resource:stamina/10".into()),
                applies_tags: vec!["$my:effect:draw/2".into(), "$your:effect:discard/2".into()]
                    .into(),
                has_tags: vec!["card:type:bag".into()].into(),
            },
        ]
    }
}

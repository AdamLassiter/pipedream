use std::{collections::BTreeMap, fs::File};

use crate::resource::combat::{
    card::{Card, Cards},
    field::{CombatPlace, CombatSide, FieldPlace},
    stats::{Condition, Debuff, Element, Resource},
};

impl Cards {
    fn dump(&self) {
        let buffer = File::create("./cards-state.yml").unwrap();
        serde_yml::to_writer(buffer, &self).unwrap();
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
                starts: CombatPlace {
                    side: CombatSide::Mine,
                    place: FieldPlace::Innate,
                },
                costs: BTreeMap::from([(Resource::Mana, 100)]),
                damages: BTreeMap::from([((CombatSide::Yours, Element::Force), 100)]),
                conditions: BTreeMap::from([((CombatSide::Yours, Condition::Debuff(Debuff::Shatter)), 100)]),
                manipulations: BTreeMap::from([((
                    CombatPlace {
                        side: CombatSide::Yours,
                        place: FieldPlace::Deck,
                    },
                    CombatPlace {
                        side: CombatSide::Mine,
                        place: FieldPlace::Hand,
                    },
                    ), 1),
                ]),
                applies_tags: BTreeMap::from([(CombatSide::Yours, vec!["card:debuff:special:omen".into()])]),
                has_tags: vec!["card:type:device".into()],
            },
            Card {
                name: "Bag of Endless Bags".into(),
                starts: CombatPlace{ side: CombatSide::Mine, place: FieldPlace::Deck },
                costs: BTreeMap::from([(Resource::Mana, 100)]),
                damages: BTreeMap::new(),
                conditions: BTreeMap::new(),
                manipulations: BTreeMap::new(),
                applies_tags: BTreeMap::new(),
                has_tags: vec!["card:type:bag".into()],
            },
        ]
    }
}

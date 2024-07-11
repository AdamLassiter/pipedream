use std::collections::BTreeMap;

use log::error;
use serde::{Deserialize, Serialize};

use crate::core::{
    action::Action,
    predicate::Predicate,
    tags::{TagKey, Tags},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cards(pub BTreeMap<String, Card>);

impl From<Vec<Card>> for Cards {
    fn from(values: Vec<Card>) -> Self {
        Self(BTreeMap::from_iter(
            values.into_iter().map(|card| (card.name.clone(), card)),
        ))
    }
}

impl Cards {
    pub fn find(&self, card: &TagKey) -> &Card {
        self.0.get(card.trailing_key()).unwrap_or_else(|| {
            error!(target:"Combat/Lookup", "Failed to find card by tag '{:?}'", card.0);
            panic!("Failed to find card by tag '{:?}'", card.0)
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub name: String,
    pub details: Vec<String>,
    pub cost: String,
    pub tags: Tags,
    pub predicate: Predicate,
    pub actions: Vec<Action>,
}

use std::collections::BTreeMap;

use log::error;
use serde::{Deserialize, Serialize};

use crate::engine::core::{
    predicate::Predicate,
    tag::{TagKey, Tags},
};

#[derive(Serialize, Deserialize)]
pub struct Cards(pub BTreeMap<String, Card>);

impl Cards {
    pub fn find(&self, card: &TagKey) -> &Card {
        self.0.get(card.trailing_key()).unwrap_or_else(|| {
            error!(target:"Combat/Lookup", "Failed to find card by tag '{:?}'", card.0);
            panic!("Failed to find card by tag '{:?}'", card.0)
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub predicate: Predicate,
    pub has_tags: Tags,
    pub applies_tags: Tags,
}

use std::collections::BTreeMap;

use log::error;
use serde::{Deserialize, Serialize};

use crate::resource::core::tag::{Tag, TagKey};

use super::{
    field::{CombatPlace, CombatSide},
    stats::{Condition, Element, Resource},
};

#[derive(Serialize, Deserialize)]
pub struct Cards(pub BTreeMap<String, Card>);

impl Cards {
    pub fn find(&self, card: &TagKey) -> &Card {
        self.0.get(card.trailing_key()).unwrap_or_else(|| {
            error!("Failed to find card by tag '{:?}'", card.0);
            panic!("Failed to find card by tag '{:?}'", card.0)
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub starts: CombatPlace,
    pub costs: BTreeMap<Resource, i64>,
    pub damages: BTreeMap<(CombatSide, Element), i64>,
    pub conditions: BTreeMap<(CombatSide, Condition), i64>,
    pub manipulations: BTreeMap<(CombatPlace, CombatPlace), u64>,
    pub applies_tags: BTreeMap<CombatSide, Vec<Tag>>,
    pub has_tags: Vec<Tag>,
}

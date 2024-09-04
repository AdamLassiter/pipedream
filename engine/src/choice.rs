use std::time::Duration;

use rusqlite::Connection;
use rusqlite_orm::orm_bind;
use serde::{Deserialize, Serialize};

use super::{description::Description, effect::Effect, image::Image, predicate::Predicate};

#[derive(Debug, Clone, Default)]
#[orm_bind ({name: "$.summary"}, [])]
pub struct Card {
    pub summary: String,
    pub cost: Option<String>,
    pub details: Vec<Description>,
    pub image: Image,
    pub predicate: Option<Predicate>,
    pub effect: Effect,
}
impl Card {
    pub fn get_card(conn: &Connection, card_id: &CardId) -> Option<Self> {
        Self::query(conn, card_id)
            .ok()
            .unwrap_or_else(|| panic!("Failed to find Card for {:?}", card_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub card: Card,
    pub selectable: bool,
}

impl Default for Choice {
    fn default() -> Self {
        Self {
            card: Default::default(),
            selectable: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Choices {
    Auto(Effect, Duration),
    Manual(Vec<Choice>),
}

impl Choices {
    pub fn manual(value: Vec<Choice>) -> Self {
        Self::Manual(value)
    }

    pub fn timed(value: Effect, duration: Duration) -> Self {
        Self::Auto(value, duration)
    }

    pub fn skip(value: Effect) -> Self {
        Self::Auto(value, Duration::from_secs(0))
    }
}

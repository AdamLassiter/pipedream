use std::time::Duration;

use rusqlite::Connection;
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize};

use super::{description::Description, effect::Effect, image::Image, predicate::Predicate};

#[derive(Debug, Clone, Default)]
#[orm_autobind]
pub struct Card {
    pub title: String,
    pub cost: Option<String>,
    pub details: Vec<Description>,
    pub image: Image,
    pub predicate: Option<Predicate>,
    pub effect: Effect,
}
impl Card {
    pub fn get_card(conn: &Connection, card_id: &CardId) -> Option<Self> {
        CardDao::select_id(conn, card_id)
            .ok()
            .and_then(|mut cards| cards.pop())
            .map(|card| card.into())
    }

    pub fn predicate_satisfied(&self, conn: &Connection) -> bool {
        self.predicate
            .as_ref()
            .map(|pred| {
                pred.test(conn)
                    .ok()
                    .unwrap_or_else(|| panic!("Failed to test Predicate for {:?}", self))
            })
            .unwrap_or(true)
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
    pub fn cards(value: Vec<Card>) -> Self {
        Self::Manual(
            value
                .into_iter()
                .map(|card| Choice {
                    card,
                    ..Default::default()
                })
                .collect(),
        )
    }

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

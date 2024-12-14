use std::time::Duration;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::{
    card::Card, description::Description, effect::Effect, image::Image, predicate::Predicate,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub title: String,
    pub cost: Option<String>,
    pub details: Vec<Description>,
    pub image: Image,
    pub predicate: Option<Predicate>,
    pub effect: Effect,
    pub selectable: bool,
}
impl Default for Choice {
    fn default() -> Self {
        Self {
            title: Default::default(),
            cost: Default::default(),
            details: Default::default(),
            image: Default::default(),
            predicate: Default::default(),
            effect: Default::default(),
            selectable: true,
        }
    }
}
impl Choice {
    pub fn predicate_satisfied(&self, conn: &Connection) -> bool {
        self.predicate
            .as_ref()
            .map(|pred| {
                pred.test(conn)
                    .unwrap_or_else(|e| panic!("Failed to test Predicate for {:?}: {}", self, e))
            })
            .unwrap_or(true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Choices {
    Auto(Effect, Duration),
    Manual(Vec<Choice>),
}

impl Choices {
    pub fn cards(value: Vec<Card>) -> Self {
        Self::Manual(value.into_iter().map(|card| card.choice).collect())
    }

    pub fn manual(value: Vec<Choice>) -> Self {
        Self::Manual(value)
    }

    pub fn cpu(value: Vec<Choice>, default: Choice) -> Self {
        let choice = value
            .into_iter()
            .find(|choice| choice.selectable)
            .unwrap_or(default);
        Self::Auto(choice.effect, Duration::from_secs(1))
    }

    pub fn timed(value: Effect, duration: Duration) -> Self {
        Self::Auto(value, duration)
    }

    pub fn skip(value: Effect) -> Self {
        Self::Auto(value, Duration::from_secs(0))
    }
}

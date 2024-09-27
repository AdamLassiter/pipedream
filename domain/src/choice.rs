use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{card::Card, effect::Effect};

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

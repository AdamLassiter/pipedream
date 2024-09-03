use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{description::Description, effect::Effect, image::Image, predicate::Predicate};

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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub summary: String,
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
            summary: Default::default(),
            cost: Default::default(),
            details: Default::default(),
            image: Default::default(),
            predicate: Default::default(),
            effect: Default::default(),
            selectable: true,
        }
    }
}

use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{predicate::Predicate, transition::Transition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choices {
    pub choices: ChoiceType,
    #[serde(default = "zero")]
    #[serde(skip_serializing)]
    pub cursor: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub summary: String,
    pub details: Vec<String>,
    pub cost: Option<String>,
    pub predicate: Option<Predicate>,
    pub effect: Transition,
    pub selectable: bool,
}

impl Default for Choice {
    fn default() -> Self {
        Self {
            summary: Default::default(),
            details: Default::default(),
            cost: Default::default(),
            predicate: Default::default(),
            effect: Default::default(),
            selectable: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChoiceType {
    Auto(Transition, Duration),
    Manual(Vec<Choice>),
}

fn zero() -> usize {
    0
}

impl From<Vec<Choice>> for Choices {
    fn from(value: Vec<Choice>) -> Self {
        Self {
            choices: ChoiceType::Manual(value),
            cursor: 0,
        }
    }
}

impl From<Transition> for Choices {
    fn from(value: Transition) -> Self {
        Self {
            choices: ChoiceType::Auto(value, Duration::from_secs(0)),
            cursor: 0,
        }
    }
}

impl From<(Transition, Duration)> for Choices {
    fn from((value, duration): (Transition, Duration)) -> Self {
        Self {
            choices: ChoiceType::Auto(value, duration),
            cursor: 0,
        }
    }
}

use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{description::Description, transition::Transition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choices {
    pub choices: ChoiceType,
    #[serde(default = "zero")]
    #[serde(skip_serializing)]
    pub cursor: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub description: Description,
    pub effect: Transition,
    pub selectable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChoiceType {
    Auto(Transition, Duration),
    Manual(Vec<Choice>),
}

fn zero() -> usize {
    0
}

impl From<Vec<(Description, Transition)>> for Choices {
    fn from(value: Vec<(Description, Transition)>) -> Self {
        Self {
            choices: ChoiceType::Manual(
                value
                    .into_iter()
                    .map(|(description, effect)| Choice {
                        description,
                        effect,
                        selectable: true,
                    })
                    .collect(),
            ),
            cursor: 0,
        }
    }
}

impl From<Vec<(Description, Transition, bool)>> for Choices {
    fn from(value: Vec<(Description, Transition, bool)>) -> Self {
        Self {
            choices: ChoiceType::Manual(
                value
                    .into_iter()
                    .map(|(description, effect, selectable)| Choice {
                        description,
                        effect,
                        selectable,
                    })
                    .collect(),
            ),
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

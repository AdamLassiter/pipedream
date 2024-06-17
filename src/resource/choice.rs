use serde::{Deserialize, Serialize};

use crate::resource::{description::Description, transition::Transition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choices {
    pub choices: Vec<Choice>,
    #[serde(default = "zero")]
    #[serde(skip_serializing)]
    pub cursor: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub description: Description,
    pub effect: Transition,
}

fn zero() -> usize {
    0
}

impl From<Vec<(Description, Transition)>> for Choices {
    fn from(value: Vec<(Description, Transition)>) -> Self {
        Choices {
            choices: value
                .into_iter()
                .map(|(description, effect)| Choice {
                    description,
                    effect,
                })
                .collect(),
            cursor: 0,
        }
    }
}

impl From<Transition> for Choices {
    fn from(value: Transition) -> Self {
        vec![("Continue".into(), value)].into()
    }
}

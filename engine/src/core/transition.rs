use serde::{Deserialize, Serialize};

use super::{action::Action, location::Location};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    #[serde(default = "none")]
    pub next: TransitionType,
    pub actions: Vec<Action>,
}

fn none() -> TransitionType {
    TransitionType::None
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    None,
    Leave,
    Enter(Location),
    Goto(Location),
    Combat(Vec<Action>),
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            next: TransitionType::None,
            actions: Default::default(),
        }
    }
}

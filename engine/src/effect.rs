use serde::{Deserialize, Serialize};

use super::{action::Action, location::Location};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Effect {
    #[serde(default = "none")]
    pub transition: Transition,
    pub actions: Vec<Action>,
}

fn none() -> Transition {
    Transition::None
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transition {
    None,
    Leave,
    Enter(Location),
    Fight(Location),
    Goto(Location),
}

impl Default for Effect {
    fn default() -> Self {
        Self {
            transition: Transition::None,
            actions: vec![],
        }
    }
}

impl Effect {
    pub fn actions(actions: Vec<Action>) -> Self {
        Self {
            actions,
            ..Default::default()
        }
    }

    pub fn transition(transition: Transition) -> Self {
        Self {
            transition,
            ..Default::default()
        }
    }
}

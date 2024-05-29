use serde::{Deserialize, Serialize};

use crate::{resource::action::Action, resource::location::Location};

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
    Pop,
    Push(Location),
    Swap(Location),
}

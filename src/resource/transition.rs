use serde::{Deserialize, Serialize};

use crate::{resource::action::Action, resource::location::Location};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideEffect {
    pub next: TransitionType,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    None,
    Pop,
    Push(Location),
    Swap(Location),
}

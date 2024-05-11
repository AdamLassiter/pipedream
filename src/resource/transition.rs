use serde::{Deserialize, Serialize};

use crate::{resource::action::Action, resource::location::Location};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub next: Location,
    pub actions: Vec<Action>,
}

use serde_derive::Serialize;

use crate::{resource::location::Location, resource::action::Action};

#[derive(Debug, Clone, Serialize)]
pub struct Transition {
    pub next: Location,
    pub actions: Vec<Action>,
}

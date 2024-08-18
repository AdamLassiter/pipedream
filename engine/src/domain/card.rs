use rusqlite_orm::orm_bind;
use serde::{Deserialize, Serialize};

use crate::core::choice::Choice;

#[derive(Clone, Debug)]
#[orm_bind ({name: "$.choice.summary"}, [])]
pub struct Card {
    pub choice: Choice,
}

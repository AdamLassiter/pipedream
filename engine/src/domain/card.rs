use serde::{Deserialize, Serialize};

use crate::core::choice::Choice;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card(Choice);

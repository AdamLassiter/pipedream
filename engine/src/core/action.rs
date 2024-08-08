use serde::{Deserialize, Serialize};

// An action is a SQL statement to be executed
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub sql: String,
}

impl Action {
    pub fn run(&self) {}
}

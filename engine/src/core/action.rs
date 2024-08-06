use serde::{Deserialize, Serialize};

// An action is a SQL statement to be executed
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action(String);

impl Action {
    pub fn run(&self) {}
}

use serde::{Deserialize, Serialize};

// A predicate is a SQL statement to be queried and compared against boolean true/1 and false/0
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Predicate {
    pub sql: String,
}

impl Predicate {
    pub fn test(&self) -> bool {
        true
    }
}

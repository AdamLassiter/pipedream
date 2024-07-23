use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Location(pub String);

impl From<&str> for Location {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

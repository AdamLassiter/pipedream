use serde::{Deserialize, Serialize};

use super::predicate::Predicate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    pub descriptor: String,
}

impl From<(Predicate, String)> for Description {
    fn from((predicate, descriptor): (Predicate, String)) -> Self {
        Description {
            predicate: Some(predicate),
            descriptor,
        }
    }
}

impl From<&str> for Description {
    fn from(descriptor: &str) -> Self {
        Description {
            predicate: None,
            descriptor: descriptor.to_string(),
        }
    }
}

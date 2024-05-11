use serde_derive::Serialize;

use super::predicate::Predicate;

#[derive(Debug, Clone, Serialize)]
pub struct Description(pub Option<Predicate>, pub String);

impl From<(Predicate, String)> for Description {
    fn from((predicate, desc): (Predicate, String)) -> Self {
        Description(Some(predicate), desc)
    }
}

impl From<&str> for Description {
    fn from(desc: &str) -> Self {
        Description(None, desc.to_string())
    }
}

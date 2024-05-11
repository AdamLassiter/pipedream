use serde_derive::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize)]
pub struct Tag(pub String);

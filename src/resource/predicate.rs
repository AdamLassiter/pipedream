use serde_derive::Serialize;

use super::tag::Tag;

#[derive(Debug, Clone, Serialize)]
pub enum Predicate {
    And(Box<Predicate>),
    Or(Box<Predicate>),
    Not(Box<Predicate>),
    Tag(Tag),
}

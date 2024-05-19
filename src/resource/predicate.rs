use serde::{Deserialize, Serialize};

use super::tag::Tag;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Predicate {
    And(Box<Vec<Predicate>>),
    Or(Box<Vec<Predicate>>),
    Not(Box<Predicate>),
    Tag(Tag),
}

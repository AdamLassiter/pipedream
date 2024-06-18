use serde::{Deserialize, Serialize};

use super::tag::Tag;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    None,
    Insert(Tag),
    Remove(Tag),
    Add(Tag),
    Subtract(Tag),
    Multiply(Tag),
    Divide(Tag),
}

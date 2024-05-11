use serde_derive::Serialize;

use super::tag::Tag;

#[derive(Debug, Clone, Serialize)]
pub enum Action {
    None,
    Insert(Tag),
    Remove(Tag),
    Add(Tag, f64),
    Subtract(Tag, f64),
    Multiply(Tag, f64),
    Divide(Tag, f64),
}

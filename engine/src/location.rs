use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Location(String);

impl Location {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self(value.into())
    }
}

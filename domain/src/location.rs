use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Location(pub String);

impl Location {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self(value.into())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocationStack(pub Vec<Location>);

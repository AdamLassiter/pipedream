use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image(pub String);

impl Image {
    fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self(value.into())
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new("resources/legacy/tile003.png")
    }
}

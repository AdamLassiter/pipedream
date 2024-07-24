use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image(pub String);

impl From<&str> for Image {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

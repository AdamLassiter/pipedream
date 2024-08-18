use serde::{Deserialize, Serialize};

use super::command::UiMode;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Location {
    pub location: String,
    pub ui_mode: UiMode,
}

impl Location {
    pub fn combat<T>(location: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            location: location.into(),
            ui_mode: UiMode::Combat,
        }
    }

    pub fn campaign<T>(location: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            location: location.into(),
            ui_mode: UiMode::Campaign,
        }
    }
}

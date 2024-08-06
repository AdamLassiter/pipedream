use serde::{Deserialize, Serialize};

use super::command::UiMode;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Location {
    pub location: String,
    pub ui_mode: UiMode,
}

impl Location {
    fn from<T>(location: T, ui_mode: UiMode) -> Self
    where
        T: Into<String>,
    {
        Self {
            location: location.into(),
            ui_mode,
        }
    }
}

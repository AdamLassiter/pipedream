use serde::{Deserialize, Serialize};

use crate::scene::Scene;
use pipedream_domain::{choice::Choices, effect::Effect};

#[derive(Debug, Clone)]
pub enum EngineCommand {
    RespondWithChoice(Effect),
    Exit,
}

#[derive(Debug, Clone)]
pub enum UiCommand {
    ShowScene(Scene),
    ShowChoices(Choices),
    ChangeMode(UiMode),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum UiMode {
    Campaign,
    Combat,
    // Inventory,
}

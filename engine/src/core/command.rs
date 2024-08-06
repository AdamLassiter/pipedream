use serde::{Deserialize, Serialize};

use super::{choice::Choices, effect::Effect, scene::Scene};

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

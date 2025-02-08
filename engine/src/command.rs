use serde::{Deserialize, Serialize};

use crate::scene::Scene;
use pipedream_domain::{
    choice::Choices, effect::Effect, image::Image, location::LocationStack, message::Message,
    player::Player, stats::Stats,
};

#[derive(Debug, Clone)]
pub enum EngineCommand {
    RespondWithChoice(Effect),
    Exit,
}

#[derive(Debug, Clone)]
pub enum UiCommand {
    ChangeMode(UiMode),
    ShowChoices(Choices),
    ShowLocation(LocationStack),
    ShowMessage(Message),
    ShowPortrait(Player, Option<Image>),
    ShowScene(Scene),
    ShowStats(Player, Option<Stats>),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum UiMode {
    Campaign,
    Combat,
    // Inventory,
}

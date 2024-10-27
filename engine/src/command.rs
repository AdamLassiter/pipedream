use serde::{Deserialize, Serialize};

use crate::scene::Scene;
use pipedream_domain::{
    choice::Choices, effect::Effect, image::Image, player::Player, stats::Stats,
};

#[derive(Debug, Clone)]
pub enum EngineCommand {
    RespondWithChoice(Effect),
    Exit,
}

#[derive(Debug, Clone)]
pub enum UiCommand {
    ShowScene(Scene),
    ShowChoices(Choices),
    ShowStats(Player, Option<Stats>),
    ShowPortrait(Player, Option<Image>),
    ChangeMode(UiMode),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum UiMode {
    Campaign,
    Combat,
    // Inventory,
}

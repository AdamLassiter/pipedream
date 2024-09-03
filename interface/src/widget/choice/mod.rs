use campaign_choice::CampaignChoices;
use combat_choice::CombatChoices;
use pipedream_engine::{choice::Choices, command::UiMode};

use crate::{Controllable, Renderable};

pub mod campaign_choice;
pub mod combat_choice;

pub enum ChoicesWidget {
    CampaignChoices(CampaignChoices),
    CombatChoices(CombatChoices),
}

impl ChoicesWidget {
    pub fn new(choices: Choices, ui_mode: &UiMode) -> Self {
        match ui_mode {
            UiMode::Campaign => Self::CampaignChoices(CampaignChoices(choices, 0)),
            UiMode::Combat => Self::CombatChoices(CombatChoices(choices, 0)),
        }
    }

    pub fn controllable(&mut self) -> &mut dyn Controllable {
        match self {
            Self::CampaignChoices(choices) => choices as &mut dyn Controllable,
            Self::CombatChoices(choices) => choices as &mut dyn Controllable,
        }
    }

    pub fn renderable(&self) -> &dyn Renderable {
        match self {
            Self::CampaignChoices(choices) => choices as &dyn Renderable,
            Self::CombatChoices(choices) => choices as &dyn Renderable,
        }
    }

    pub fn choices(&self) -> &Choices {
        match self {
            Self::CampaignChoices(CampaignChoices(choices, _)) => choices,
            Self::CombatChoices(CombatChoices(choices, _)) => choices,
        }
    }

    pub fn cursor(&self) -> &usize {
        match self {
            Self::CampaignChoices(CampaignChoices(_, cursor)) => cursor,
            Self::CombatChoices(CombatChoices(_, cursor)) => cursor,
        }
    }
}

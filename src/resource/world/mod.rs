use campaign_world::CampaignWorld;

use crate::engine::coordinator::combat_coordinator::CombatCoordinator;

pub mod combat_world;
pub mod campaign_world;

pub enum World {
    CampaignWorld(CampaignWorld),
    CombatWorld(CombatCoordinator),
}

use static_world::CampaignWorld;

use crate::engine::coordinator::combat_coordinator::CombatCoordinator;

pub mod dynamic_world;
pub mod static_world;

pub enum World {
    CampaignWorld(CampaignWorld),
    CombatWorld(CombatCoordinator),
}

pub mod campaign_state_machine;
pub mod campaign_world;
pub mod combat_state_machine;
pub mod combat_world;
pub mod tag_engine;

use campaign_state_machine::CampaignStateMachine;
use campaign_world::CampaignWorld;
use combat_state_machine::CombatStateMachine;
use combat_world::CombatWorld;

pub enum StateMachine {
    CampaignStateMachine(CampaignStateMachine),
    CombatStateMachine(CombatStateMachine),
}

pub enum World {
    CampaignWorld(CampaignWorld),
    CombatWorld(CombatWorld),
}

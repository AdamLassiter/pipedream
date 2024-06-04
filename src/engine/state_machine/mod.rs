pub mod campaign_state_machine;
pub mod combat_state_machine;

use campaign_state_machine::CampaignStateMachine;
use combat_state_machine::CombatStateMachine;

pub enum StateMachine {
    CampaignStateMachine(CampaignStateMachine),
    CombatStateMachine(CombatStateMachine),
}

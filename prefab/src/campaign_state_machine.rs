use pipedream_engine::state::{
    campaign_state_machine::CampaignStateMachine, combat_state_machine::CombatStateMachine,
    combat_world::CombatWorld,
};

use crate::{combat_world::COMBAT_INIT, tag_engine::into_combat, Generatable};

pub fn campaign_exporter(campaign_machine: &CampaignStateMachine) -> CombatStateMachine {
    CombatStateMachine::new(
        CombatWorld::generate(),
        into_combat(&campaign_machine.tag_engine),
        COMBAT_INIT.clone(),
        campaign_importer,
    )
}

pub fn campaign_importer(
    _combat_machine: CombatStateMachine,
    _campaign_machine: &mut CampaignStateMachine,
) {
    todo!()
}

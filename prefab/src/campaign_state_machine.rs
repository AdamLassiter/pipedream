use pipedream_engine::{
    core::{
        action::Action,
        tags::TagKey,
        transition::{Transition, TransitionType},
    },
    state::{
        campaign_state_machine::CampaignStateMachine, combat_state_machine::CombatStateMachine,
        combat_world::CombatWorld,
    },
};

use crate::{
    combat_world::COMBAT_INIT,
    tag_engine::{from_combat, into_combat},
    Generatable,
};

pub fn campaign_exporter(campaign_machine: &CampaignStateMachine) -> CombatStateMachine {
    CombatStateMachine::new(
        CombatWorld::generate(),
        into_combat(&campaign_machine.tag_engine),
        COMBAT_INIT.clone(),
        combat_exporter,
    )
}

pub fn combat_exporter(combat_machine: &CombatStateMachine) -> Transition {
    Transition {
        next: TransitionType::None,
        actions: from_combat(&combat_machine.tag_engine)
            .tags
            .find(&TagKey("".into()))
            .into_iter()
            .map(Action::Insert)
            .collect::<Vec<_>>(),
    }
}

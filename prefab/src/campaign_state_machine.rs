use pipedream_engine::{
    action::Action,
    effect::{Effect, Transition},
    state_machine::StateMachine,
};

use crate::{
    combat_world::COMBAT_INIT,
    tag_engine::{from_combat, into_combat},
    Generatable,
};

pub fn campaign_exporter(campaign_machine: &StateMachine) -> StateMachine {
    StateMachine::new(
        into_combat(&campaign_machine.tag_engine),
        COMBAT_INIT.clone(),
        combat_exporter,
    )
}

pub fn combat_exporter(combat_machine: &StateMachine) -> Effect {
    Effect {
        transition: Transition::None,
        actions: from_combat(&combat_machine.tag_engine)
            .tags
            .find(&TagKey("".into()))
            .into_iter()
            .map(Action::Insert)
            .collect::<Vec<_>>(),
    }
}

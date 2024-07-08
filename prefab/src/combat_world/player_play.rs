use log::debug;

use crate::combat_world::{PLAYER_DAMG, PLAYER_PLAY};
use pipedream_engine::{
    combat::{entity::Ent, target::Tgt},
    core::{
        action::Action,
        scene::Scene,
        state::State,
        tags::Tag,
        transition::{Transition, TransitionType},
    },
    state::combat_state_machine::CombatStateMachine,
};

pub fn player_play(machine: &CombatStateMachine) -> State {
    let player_hand_slice = machine.tag_engine.find(&Tgt::Me.ent(Ent::Hand));
    debug!(target:"Combat/Hand", "{:?}", player_hand_slice);

    State {
        location: PLAYER_PLAY.clone(),
        scene: Scene {
            descriptions: vec!["Play".into()],
        },
        options: player_hand_slice
            .iter()
            .map(|Tag { key: card, .. }| machine.combat_world.cards.find(card))
            .map(|card_data| {
                let selectable = machine.tag_engine.satisfies(&card_data.predicate);
                (
                    format!("Play {:?} [{}]", card_data.name, card_data.predicate).into(),
                    Transition {
                        next: TransitionType::Goto(PLAYER_DAMG.clone()),
                        actions: card_data
                            .actions
                            .clone()
                            .into_iter()
                            .chain(vec![Action::Subtract(
                                format!("{}:{}:{}", Tgt::Me, Ent::Hand, card_data.name).into(),
                            )])
                            .collect(),
                    },
                    selectable,
                )
            })
            .collect::<Vec<_>>()
            .into(),
    }
}

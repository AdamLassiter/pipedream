use log::debug;

use crate::{
    engine::{
        core::{
            action::Action, scene::Scene, state::State, tag::Tag, transition::{Transition, TransitionType}
        },
        state::{combat_state_machine::*, combat_world::*},
    },
    prefab::combat_world::{PLAYER_DAMAGE, PLAYER_PLAY},
};

impl CombatWorld {
    pub fn player_play_phase(machine: &CombatStateMachine) -> State {
        let player_hand_slice = machine.tag_engine.find(&MY_HAND);
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
                            next: TransitionType::Goto(PLAYER_DAMAGE.clone()),
                            actions: card_data.actions.clone().into_iter().chain(vec![
                                Action::Subtract(format!("{}:{}", MY_HAND.0, card_data.name).into())
                            ]).collect(),
                        },
                        selectable,
                    )
                })
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

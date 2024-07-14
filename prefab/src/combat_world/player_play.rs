use std::iter::repeat_n;

use pipedream_engine::log::debug;

use crate::combat_world::{PLAYER_DAMAGE, PLAYER_PLAY};
use pipedream_engine::{
    combat::{card::Card, entity::Ent, target::Tgt},
    core::{
        action::Action,
        choice::Choice,
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
            .map(|Tag { key: card, value }| (machine.combat_world.cards.find(card), value))
            .flat_map(
                |(
                    Card {
                        name,
                        image,
                        details,
                        cost,
                        predicate,
                        actions,
                        ..
                    },
                    value,
                )| {
                    let selectable = machine.tag_engine.satisfies(predicate);
                    let choice = Choice {
                        summary: name.clone(),
                        image: Some(image.clone()),
                        details: details.clone(),
                        cost: Some(cost.clone()),
                        predicate: Some(predicate.clone()),
                        effect: Transition {
                            next: TransitionType::Goto(PLAYER_DAMAGE.clone()),
                            actions: actions
                                .clone()
                                .into_iter()
                                .chain(vec![Action::Subtract(
                                    format!("{}:{}:{}", Tgt::Me, Ent::Hand, name).into(),
                                )])
                                .collect(),
                        },
                        selectable,
                    };
                    repeat_n(
                        choice,
                        value
                            .number()
                            .expect("Failed to get Number of cards in hand")
                            .to_num(),
                    )
                },
            )
            .collect::<Vec<_>>()
            .into(),
    }
}

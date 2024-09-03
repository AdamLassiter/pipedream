use std::iter::repeat_n;

use pipedream_engine::{description::Description, log::debug};

use crate::combat_world::{HUMAN_DAMAGE, HUMAN_PLAY};
use pipedream_domain::{card::Card, entity::Ent, target::Target};
use pipedream_engine::{
    action::Action,
    choice::Choice,
    effect::{Effect, Transition},
    scene::Scene,
    state::combat_state_machine::StateMachine,
    state::State,
    tag::Tag,
};

pub fn player_play(machine: &StateMachine) -> State {
    let player_hand_slice = machine.tag_engine.find(&Target::Me.ent(Ent::Hand));
    debug!(target:"Prefab/Combat/Hand", "{:?}", player_hand_slice);

    State {
        location: HUMAN_PLAY.clone(),
        scene: Scene {
            descriptions: vec![Description::always("Play")],
        },
        choices: player_hand_slice
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
                        effect: Effect {
                            transition: Transition::Goto(HUMAN_DAMAGE.clone()),
                            actions: actions
                                .clone()
                                .into_iter()
                                .chain(vec![Action::Subtract(
                                    format!("{}:{}:{}", Target::Me, Ent::Hand, name).into(),
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

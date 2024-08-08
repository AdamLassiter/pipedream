use pipedream_engine::core::choice::Choices;
use pipedream_engine::log::debug;

use pipedream_engine::{
    core::state_machine::StateMachine,
    core::{
        effect::{Effect, Transition},
        scene::Scene,
        state::State,
        tag::Tag,
    },
    domain::{entity::Ent, target::Target},
};

use crate::combat_world::{COMBAT_DEFEAT, COMBAT_END, COMBAT_VICTORY, PLAYER_PLAY};

pub fn combat_end(machine: &StateMachine) -> State {
    let player_health_slice = machine
        .tag_engine
        .find(&Target::Player.ent(Ent::ResourceHealth));
    let enemy_health_slice = machine
        .tag_engine
        .find(&Target::Enemy.ent(Ent::ResourceHealth));

    debug!(target:"Prefab/Combat/End", "{:?} vs {:?}", player_health_slice, enemy_health_slice);

    let next = match player_health_slice.first() {
        None => &COMBAT_DEFEAT,
        Some(Tag {
            value: TagValue::Number(health),
            ..
        }) if !health.is_positive() => &COMBAT_DEFEAT,
        _ => match enemy_health_slice.first() {
            None => &COMBAT_VICTORY,
            Some(Tag {
                value: TagValue::Number(health),
                ..
            }) if !health.is_positive() => &COMBAT_VICTORY,
            _ => &PLAYER_PLAY,
        },
    };

    State {
        location: COMBAT_END.clone(),
        scene: Scene {
            descriptions: vec![],
        },
        choices: Choices::skip(Effect {
            transition: Transition::Goto((*next).clone()),
            actions: vec![],
        }),
    }
}

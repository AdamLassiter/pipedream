use log::debug;

use pipedream_engine::{
    combat::{entity::Ent, target::Tgt},
    core::{
        scene::Scene,
        state::State,
        tags::{Tag, TagValue},
        transition::{Transition, TransitionType},
    },
    state::combat_state_machine::CombatStateMachine,
};

use crate::combat_world::{COMBAT_DEFEAT, COMBAT_END, COMBAT_VICTORY, PLAYER_PLAY};

pub fn combat_end(machine: &CombatStateMachine) -> State {
    let player_health_slice = machine
        .tag_engine
        .find(&Tgt::Player.ent(Ent::ResourceHealth));
    let enemy_health_slice = machine
        .tag_engine
        .find(&Tgt::Enemy.ent(Ent::ResourceHealth));

    debug!(target:"Combat/End", "{:?} {:?}", player_health_slice, enemy_health_slice);

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
        options: Transition {
            next: TransitionType::Goto((*next).clone()),
            actions: vec![],
        }
        .into(),
    }
}

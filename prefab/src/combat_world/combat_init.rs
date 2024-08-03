use std::time::Duration;

use pipedream_engine::log::debug;

use crate::{
    combat_world::{COMBAT_INIT, PLAYER_DRAW},
    tags::{ME_REF, YOU_REF},
};
use pipedream_engine::{
    game::{entity::Ent, target::Tgt},
    core::{
        action::Action,
        scene::Scene,
        state::State,
        tags::{Tag, TagKey},
        transition::{Transition, TransitionType},
    },
    state::combat_state_machine::CombatStateMachine,
};

pub fn combat_init(machine: &CombatStateMachine) -> State {
    let enemy_name_slice = machine.tag_engine.find(&Tgt::Enemy.ent(Ent::Name));
    debug!(target:"Prefab/Combat/Init", "{:?}", enemy_name_slice);

    let Tag { key: enemy, .. } = enemy_name_slice
        .first()
        .expect("Failed to find enemy name slice");
    let enemy_data = machine.combat_world.npcs.find(enemy);

    let initial_actions = vec![
        format!("{}={}", ME_REF.0, Tgt::Player).into(),
        format!("{}={}", YOU_REF.0, Tgt::Enemy).into(),
    ]
    .into_iter()
    .chain(enemy_data.tags.find(&TagKey("".to_string())))
    .map(Action::Insert)
    .collect();

    State {
        location: COMBAT_INIT.clone(),
        scene: Scene {
            descriptions: vec![
                "A challenger appears!".into(),
                format!("{:?} is looking for a fight", enemy_data.name).into(),
            ],
        },
        options: (
            Transition {
                next: TransitionType::Goto(PLAYER_DRAW.clone()),
                actions: initial_actions,
            },
            Duration::from_secs(2),
        )
            .into(),
    }
}

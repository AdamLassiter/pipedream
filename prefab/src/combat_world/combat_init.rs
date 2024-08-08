use std::time::Duration;

use pipedream_engine::{core::{choice::Choices, description::Description}, log::debug};

use crate::{
    combat_world::{COMBAT_INIT, PLAYER_DRAW},
    tags::{ME_REF, YOU_REF},
};
use pipedream_engine::{
    core::{
        action::Action,
        effect::{Effect, Transition},
        scene::Scene,
        state::State,
        tag::{Tag, TagKey},
    },
    domain::{entity::Ent, target::Target},
    state::combat_state_machine::CombatStateMachine,
};

pub fn combat_init(machine: &CombatStateMachine) -> State {
    let enemy_name_slice = machine.tag_engine.find(&Target::Enemy.ent(Ent::Name));
    debug!(target:"Prefab/Combat/Init", "{:?}", enemy_name_slice);

    let Tag { key: enemy, .. } = enemy_name_slice
        .first()
        .expect("Failed to find enemy name slice");
    let enemy_data = machine.combat_world.npcs.find(enemy);

    let initial_actions = vec![
        format!("{}={}", ME_REF.0, Target::Player).into(),
        format!("{}={}", YOU_REF.0, Target::Enemy).into(),
    ]
    .into_iter()
    .chain(enemy_data.tags.find(&TagKey("".to_string())))
    .map(Action::Insert)
    .collect();

    State {
        location: COMBAT_INIT.clone(),
        scene: Scene {
            descriptions: vec![
                Description::always("A challenger appears!"),
                Description::always(format!("{:?} is looking for a fight", enemy_data.name)),
            ],
        },
        choices: Choices::timed(
            Effect {
                transition: Transition::Goto(PLAYER_DRAW.clone()),
                actions: initial_actions,
            },
            Duration::from_secs(2),
        ),
    }
}

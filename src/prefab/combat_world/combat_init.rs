use std::time::Duration;

use log::debug;

use crate::{
    engine::{
        core::{
            action::Action,
            scene::Scene,
            state::State,
            tag::Tag,
            transition::{Transition, TransitionType},
        },
        state::{combat_state_machine::CombatStateMachine, combat_world::CombatWorld},
    },
    prefab::{
        combat_world::{COMBAT_INIT, PLAYER_DRAW},
        tag_engine::Ent,
        tags::{Tgt, ME_REF, YOU_REF},
    },
};

impl CombatWorld {
    pub fn combat_init_phase(machine: &CombatStateMachine) -> State {
        let enemy_name_slice = machine.tag_engine.find(&Tgt::Enemy.ent(Ent::Name));
        debug!(target:"Combat/Init", "{:?}", enemy_name_slice);

        let Tag { key: enemy, .. } = enemy_name_slice
            .first()
            .expect("Failed to find enemy name slice");
        let enemy_data = machine.combat_world.npcs.find(enemy);

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
                    actions: vec![
                        Action::Insert(format!("{}/{}", ME_REF.0, Tgt::Player).into()),
                        Action::Insert(format!("{}/{}", YOU_REF.0, Tgt::Enemy).into()),
                    ],
                },
                Duration::from_secs(2),
            )
                .into(),
        }
    }
}

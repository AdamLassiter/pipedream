use pipedream_engine::{
    core::{choice::Choices, state_machine::StateMachine},
    domain::{
        encounter::Player,
        stats::{Assistance, Element, Resistance, Resource, Stat, StatChange},
    },
    log::debug,
};

use crate::combat_world::{COMBAT_END, HUMAN_DAMAGE};
use pipedream_engine::core::{
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
};

fn calculate_damage(assist_stat: f64, resist_stat: f64, damage_val: f64) -> f64 {
    (assist_stat / resist_stat).sqrt() * damage_val
}

pub fn player_apply_stats(player: &Player, machine: &StateMachine) -> State {
    let stat_changes = machine.get_target_stat_changes(player);
    debug!(target:"Prefab/Combat/ApplyStats", "{:?} {:?}", player, stat_changes);

    machine.update_character(player, |target_char| {
        stat_changes.into_iter().for_each(
            |StatChange {
                 stat,
                 source,
                 change,
                 ..
             }| {
                match stat {
                    Stat::Element(Element::Bludgeoning) => {
                        let source_char = machine.get_character(&source);
                        let assistance = *source_char
                            .stats
                            .assisstances
                            .get(&Assistance::Strength)
                            .unwrap_or_else(|| {
                                panic!("Failed to get {:?} for {:?}", Assistance::Strength, source)
                            });
                        let resistance = *target_char
                            .stats
                            .resistances
                            .get(&Resistance::Endurance)
                            .unwrap_or_else(|| {
                                panic!("Failed to get {:?} for {:?}", Resistance::Endurance, player)
                            });
                        let damage = calculate_damage(assistance, resistance, change);
                        target_char
                            .stats
                            .resources
                            .get_mut(&Resource::Health)
                            .map(|health| *health -= damage);
                    }
                    _ => todo!(),
                }
            },
        )
    });

    State {
        location: HUMAN_DAMAGE.clone(),
        scene: Scene {
            descriptions: vec![],
        },
        choices: Choices::skip(Effect {
            transition: Transition::Goto(COMBAT_END.clone()),
            actions: vec![],
        }),
    }
}

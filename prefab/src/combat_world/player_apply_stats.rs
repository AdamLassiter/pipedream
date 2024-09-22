use log::debug;
use pipedream_domain::player::PlayerCharacter;
use pipedream_domain::{
    player::Player,
    stats::{Assistance, Element, Resistance, Resource, Stat, StatChange},
};
use pipedream_engine::command::UiMode;
use pipedream_engine::{choice::Choices, state_machine::StateMachine};

use crate::combat_world::{COMBAT_END, HUMAN_DAMAGE};
use pipedream_engine::{
    effect::{Effect, Transition},
    scene::Scene,
    state::State,
};

fn calculate_damage(assist_stat: f64, resist_stat: f64, damage_val: f64) -> f64 {
    (assist_stat / resist_stat).sqrt() * damage_val
}

pub fn player_apply_stats(player: &Player, machine: &StateMachine) -> State {
    let stat_changes = StatChange::find_target(&machine.conn, player);
    debug!(target:"Prefab/Combat/ApplyStats", "{:?} {:?}", player, stat_changes);

    let transition = Transition::Goto(COMBAT_END.clone());
    let action =
        PlayerCharacter::update_player_character(&machine.conn, player, |mut target_char| {
            stat_changes.iter().for_each(
                |StatChange {
                     stat,
                     source,
                     change,
                     ..
                 }| {
                    match stat {
                        Stat::Element(Element::Bludgeoning) => {
                            let (_, source_char) =
                                PlayerCharacter::get_player_character(&machine.conn, source);
                            let assistance = *source_char
                                .stats
                                .assistances
                                .get(&Assistance::Strength)
                                .unwrap_or_else(|| {
                                    panic!(
                                        "Failed to get {:?} for {:?}",
                                        Assistance::Strength,
                                        source
                                    )
                                });
                            let resistance = *target_char
                                .stats
                                .resistances
                                .get(&Resistance::Endurance)
                                .unwrap_or_else(|| {
                                    panic!(
                                        "Failed to get {:?} for {:?}",
                                        Resistance::Endurance,
                                        player
                                    )
                                });
                            let damage = calculate_damage(
                                assistance as f64,
                                resistance as f64,
                                *change as f64,
                            ) as i64;
                            let max_health = *target_char
                                .stats
                                .max_resources
                                .get(&Resource::Health)
                                .expect("Failed to get Player max health");
                            if let Some(health) =
                                target_char.stats.resources.get_mut(&Resource::Health)
                            {
                                *health = (*health - damage).clamp(0, max_health as i64);
                            }
                        }
                        _ => todo!(),
                    }
                },
            );
            target_char
        });

    State {
        location: HUMAN_DAMAGE.clone(),
        scene: Scene {
            descriptions: vec![],
        },
        choices: Choices::skip(Effect {
            transition,
            actions: vec![action],
        }),
        ui_mode: UiMode::Combat,
    }
}

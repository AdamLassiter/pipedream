use log::debug;

use crate::combat_world::{CPU_DAMAGE, CPU_PLAY, HUMAN_DAMAGE, HUMAN_PLAY};
use pipedream_domain::{
    choice::Choices,
    effect::{Effect, Transition},
    player::{Player, PlayerCharacter},
    stats::{Assistance, Element, Resistance, Resource, Stat, StatChange},
};
use pipedream_engine::{command::UiMode, scene::Scene, state::State, state_machine::StateMachine};

fn calculate_damage(assist_stat: f64, resist_stat: f64, damage_val: f64) -> f64 {
    (assist_stat / resist_stat).sqrt() * damage_val
}

pub fn player_damage(player: &Player, machine: &StateMachine) -> State {
    let stat_changes = StatChange::find_target(&machine.conn, player);
    debug!(target:"Prefab/Combat/ApplyStats", "{:?} {:?}", player, stat_changes);

    let current_location = match player {
        Player::Human => HUMAN_DAMAGE.clone(),
        Player::Cpu => CPU_DAMAGE.clone(),
        Player::World => unimplemented!(),
    };
    let next_location = match player {
        Player::Human => HUMAN_PLAY.clone(),
        Player::Cpu => CPU_PLAY.clone(),
        Player::World => unimplemented!(),
    };

    // let end_transition = Transition::Goto(COMBAT_END.clone());
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
        location: current_location,
        scene: Scene {
            descriptions: vec![],
        },
        choices: Choices::skip(Effect {
            transition: Transition::Goto(next_location),
            actions: vec![action],
        }),
        ui_mode: UiMode::Combat,
    }
}

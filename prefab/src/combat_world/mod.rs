mod combat_defeat;
mod combat_end;
mod combat_start;
mod combat_victory;
mod player_damage;
mod player_draw;
mod player_play;
mod turn_end;
mod turn_start;

use std::collections::BTreeMap;

use pipedream_domain::{location::Location, player::Player};
use pipedream_engine::state::DynamicStateFn;

use combat_defeat::combat_defeat;
use combat_end::combat_end;
use combat_start::combat_start;
use combat_victory::combat_victory;
use player_damage::player_damage;
use player_draw::player_draw;
use player_play::player_play;
use turn_end::turn_end;
use turn_start::turn_start;

use crate::{Generatable, Static};

pub static COMBAT_START: Static<Location> = Static::new(|| Location::new("combat:start"));
pub static COMBAT_END: Static<Location> = Static::new(|| Location::new("combat:end"));
pub static COMBAT_VICTORY: Static<Location> = Static::new(|| Location::new("combat:victory"));
pub static COMBAT_DEFEAT: Static<Location> = Static::new(|| Location::new("combat:defeat"));

pub static HUMAN_START: Static<Location> = Static::new(|| Location::new("human:start"));
pub static HUMAN_DRAW: Static<Location> = Static::new(|| Location::new("human:draw"));
pub static HUMAN_PLAY: Static<Location> = Static::new(|| Location::new("human:play"));
pub static HUMAN_DAMAGE: Static<Location> = Static::new(|| Location::new("human:damage"));
pub static HUMAN_END: Static<Location> = Static::new(|| Location::new("human:end"));

pub static CPU_START: Static<Location> = Static::new(|| Location::new("cpu:start"));
pub static CPU_DRAW: Static<Location> = Static::new(|| Location::new("cpu:draw"));
pub static CPU_PLAY: Static<Location> = Static::new(|| Location::new("cpu:play"));
pub static CPU_DAMAGE: Static<Location> = Static::new(|| Location::new("cpu:damage"));
pub static CPU_END: Static<Location> = Static::new(|| Location::new("cpu:end"));

impl Generatable for BTreeMap<Location, DynamicStateFn> {
    fn generate() -> Self {
        BTreeMap::from_iter(vec![
            // Global
            (COMBAT_START.clone(), DynamicStateFn::new(combat_start)),
            (COMBAT_END.clone(), DynamicStateFn::new(combat_end)),
            (COMBAT_VICTORY.clone(), DynamicStateFn::new(combat_victory)),
            (COMBAT_DEFEAT.clone(), DynamicStateFn::new(combat_defeat)),
            // Human
            (
                HUMAN_START.clone(),
                DynamicStateFn::new(|machine| turn_start(&Player::Human, machine)),
            ),
            (
                HUMAN_DRAW.clone(),
                DynamicStateFn::new(|machine| player_draw(&Player::Human, machine)),
            ),
            (
                HUMAN_PLAY.clone(),
                DynamicStateFn::new(|machine| player_play(&Player::Human, machine)),
            ),
            (
                HUMAN_DAMAGE.clone(),
                DynamicStateFn::new(|machine| player_damage(&Player::Human, machine)),
            ),
            (
                HUMAN_START.clone(),
                DynamicStateFn::new(|machine| turn_end(&Player::Human, machine)),
            ),
            // Cpu
            (
                CPU_START.clone(),
                DynamicStateFn::new(|machine| turn_start(&Player::Cpu, machine)),
            ),
            (
                CPU_DRAW.clone(),
                DynamicStateFn::new(|machine| player_draw(&Player::Cpu, machine)),
            ),
            (
                CPU_PLAY.clone(),
                DynamicStateFn::new(|machine| player_play(&Player::Cpu, machine)),
            ),
            (
                CPU_DAMAGE.clone(),
                DynamicStateFn::new(|machine| player_damage(&Player::Cpu, machine)),
            ),
            (
                CPU_START.clone(),
                DynamicStateFn::new(|machine| turn_end(&Player::Cpu, machine)),
            ),
        ])
    }
}

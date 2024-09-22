mod combat_defeat;
mod combat_end;
mod combat_init;
mod combat_victory;
mod player_apply_stats;
mod player_draw;
mod player_play;
mod turn_end;
mod turn_start;

use std::collections::BTreeMap;

use pipedream_domain::player::Player;
use pipedream_engine::{location::Location, state::DynamicStateFn};

use combat_defeat::combat_defeat;
use combat_end::combat_end;
use combat_init::combat_init;
use combat_victory::combat_victory;
use player_apply_stats::player_apply_stats;
use player_draw::player_draw;
use player_play::player_play;

use crate::{Generatable, Static};

pub static COMBAT_INIT: Static<Location> = Static::new(|| Location::new("combat:init"));
pub static COMBAT_END: Static<Location> = Static::new(|| Location::new("combat:end"));
pub static COMBAT_VICTORY: Static<Location> = Static::new(|| Location::new("combat:victory"));
pub static COMBAT_DEFEAT: Static<Location> = Static::new(|| Location::new("combat:defeat"));

pub static HUMAN_DRAW: Static<Location> = Static::new(|| Location::new("human:draw"));
pub static HUMAN_PLAY: Static<Location> = Static::new(|| Location::new("human:play"));
pub static HUMAN_DAMAGE: Static<Location> = Static::new(|| Location::new("human:damage"));

pub static CPU_DRAW: Static<Location> = Static::new(|| Location::new("cpu:draw"));
pub static CPU_PLAY: Static<Location> = Static::new(|| Location::new("cpu:play"));
pub static CPU_DAMAGE: Static<Location> = Static::new(|| Location::new("cpu:damage"));

impl Generatable for BTreeMap<Location, DynamicStateFn> {
    fn generate() -> Self {
        BTreeMap::from_iter(vec![
            (COMBAT_INIT.clone(), DynamicStateFn::new(combat_init)),
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
                DynamicStateFn::new(|machine| player_apply_stats(&Player::Human, machine)),
            ),
            (COMBAT_END.clone(), DynamicStateFn::new(combat_end)),
            (COMBAT_VICTORY.clone(), DynamicStateFn::new(combat_victory)),
            (COMBAT_DEFEAT.clone(), DynamicStateFn::new(combat_defeat)),
        ])
    }
}

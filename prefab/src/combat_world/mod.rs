mod combat_defeat;
mod combat_end;
mod combat_init;
mod combat_victory;
mod player_apply_stats;
mod player_draw;
mod player_play;

use std::{collections::BTreeMap, sync::LazyLock};

use combat_defeat::combat_defeat;
use combat_end::combat_end;
use combat_init::combat_init;
use combat_victory::combat_victory;
use pipedream_engine::{core::{location::Location, state::DynamicStateFn}, domain::encounter::Player};
use player_apply_stats::player_apply_stats;
use player_draw::player_draw;
use player_play::player_play;

use crate::Generatable;

pub type Static<T> = LazyLock<T>;

pub static COMBAT_INIT: Static<Location> = Static::new(|| Location::combat("combat:init"));
pub static COMBAT_END: Static<Location> = Static::new(|| Location::combat("combat:end"));
pub static COMBAT_VICTORY: Static<Location> = Static::new(|| Location::combat("combat:victory"));
pub static COMBAT_DEFEAT: Static<Location> = Static::new(|| Location::combat("combat:defeat"));

pub static HUMAN_DRAW: Static<Location> = Static::new(|| Location::combat("human:draw"));
pub static HUMAN_PLAY: Static<Location> = Static::new(|| Location::combat("human:play"));
pub static HUMAN_DAMAGE: Static<Location> = Static::new(|| Location::combat("human:damage"));

pub static CPU_DRAW: Static<Location> = Static::new(|| Location::combat("cpu:draw"));
pub static CPU_PLAY: Static<Location> = Static::new(|| Location::combat("cpu:play"));
pub static CPU_DAMAGE: Static<Location> = Static::new(|| Location::combat("cpu:damage"));

impl Generatable for BTreeMap<Location, DynamicStateFn> {
    fn generate() -> Self {
        BTreeMap::from_iter(vec![
            (COMBAT_INIT.clone(), DynamicStateFn::new(combat_init)),
            (HUMAN_DRAW.clone(), DynamicStateFn::new(player_draw)),
            (HUMAN_PLAY.clone(), DynamicStateFn::new(player_play)),
            (HUMAN_DAMAGE.clone(), DynamicStateFn::new(|machine| player_apply_stats(&Player::Human, machine))),
            (COMBAT_END.clone(), DynamicStateFn::new(combat_end)),
            (COMBAT_VICTORY.clone(), DynamicStateFn::new(combat_victory)),
            (COMBAT_DEFEAT.clone(), DynamicStateFn::new(combat_defeat)),
        ])
    }
}

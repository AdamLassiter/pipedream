mod combat_end;
mod combat_init;
mod player_damage;
mod player_draw;
mod player_play;

use std::collections::BTreeMap;

use combat_end::combat_end;
use combat_init::combat_init;
use pipedream_engine::{
    combat::{card::Cards, npc::Npcs},
    core::{location::Location, tags::Static},
    state::combat_world::{CombatWorld, DynamicStateFn},
};
use player_damage::player_damamge;
use player_draw::player_draw;
use player_play::player_play;

use crate::Generatable;

pub static COMBAT_INIT: Static<Location> = Static::new(|| "combat:init".into());
pub static COMBAT_END: Static<Location> = Static::new(|| "combat:end".into());
pub static COMBAT_VICTORY: Static<Location> = Static::new(|| "combat:victory".into());
pub static COMBAT_DEFEAT: Static<Location> = Static::new(|| "combat:defeat".into());

pub static PLAYER_DRAW: Static<Location> = Static::new(|| "player:draw".into());
pub static PLAYER_PLAY: Static<Location> = Static::new(|| "player:play".into());
pub static PLAYER_DAMG: Static<Location> = Static::new(|| "player:damage".into());

pub static ENEMY_DRAW: Static<Location> = Static::new(|| "enemy:draw".into());
pub static ENEMY_PLAY: Static<Location> = Static::new(|| "enemy:play".into());
pub static ENEMY_DAMAGE: Static<Location> = Static::new(|| "enemy:damage".into());

impl Generatable for CombatWorld {
    fn generate() -> Self {
        let states = {
            BTreeMap::from_iter(vec![
                (COMBAT_INIT.clone(), DynamicStateFn::new(combat_init)),
                (PLAYER_DRAW.clone(), DynamicStateFn::new(player_draw)),
                (PLAYER_PLAY.clone(), DynamicStateFn::new(player_play)),
                (PLAYER_DAMG.clone(), DynamicStateFn::new(player_damamge)),
                (COMBAT_END.clone(), DynamicStateFn::new(combat_end)),
            ])
        };

        Self {
            states,
            cards: Cards::generate(),
            npcs: Npcs::generate(),
        }
    }
}

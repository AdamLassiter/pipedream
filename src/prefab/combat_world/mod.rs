mod combat_end;
mod combat_init;
mod player_damage;
mod player_draw;
mod player_play;

use std::collections::BTreeMap;

use crate::engine::{
    combat::{card::Cards, npc::Npcs},
    core::{location::Location, tag::Static},
    state::combat_world::{CombatWorld, DynamicStateFn},
};

pub static COMBAT_INIT: Static<Location> = Static::new(|| "combat:init".into());
pub static COMBAT_END: Static<Location> = Static::new(|| "combat:end".into());
pub static COMBAT_VICTORY: Static<Location> = Static::new(|| "combat:victory".into());
pub static COMBAT_DEFEAT: Static<Location> = Static::new(|| "combat:defeat".into());

pub static PLAYER_DRAW: Static<Location> = Static::new(|| "player:draw".into());
pub static PLAYER_PLAY: Static<Location> = Static::new(|| "player:play".into());
pub static PLAYER_DAMAGE: Static<Location> = Static::new(|| "player:damage".into());

pub static ENEMY_DRAW: Static<Location> = Static::new(|| "enemy:draw".into());
pub static ENEMY_PLAY: Static<Location> = Static::new(|| "enemy:play".into());
pub static ENEMY_DAMAGE: Static<Location> = Static::new(|| "enemy:damage".into());

impl CombatWorld {
    pub fn generate() -> Self {
        let states = {
            BTreeMap::from_iter(vec![
                (
                    COMBAT_INIT.clone(),
                    DynamicStateFn::new(Self::combat_init_phase),
                ),
                (
                    PLAYER_DRAW.clone(),
                    DynamicStateFn::new(Self::player_draw_phase),
                ),
                (
                    PLAYER_PLAY.clone(),
                    DynamicStateFn::new(Self::player_play_phase),
                ),
                (
                    PLAYER_DAMAGE.clone(),
                    DynamicStateFn::new(Self::player_damamge_phase),
                ),
                (
                    COMBAT_END.clone(),
                    DynamicStateFn::new(Self::combat_end_phase),
                ),
            ])
        };

        Self {
            states,
            cards: Cards::generate(),
            npcs: Npcs::generate(),
        }
    }
}

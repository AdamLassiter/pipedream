pub mod combat_init;
pub mod player_draw;
pub mod player_play;
pub mod player_damage;

use std::collections::BTreeMap;

use crate::engine::{
    combat::{card::Cards, npc::Npcs},
    core::{
        location::Location,
        tag::{Static, TagKey},
    },
    state::combat_world::*,
};

pub static COMBAT_INIT: Static<Location> = Static::new(|| "combat:init".into());
pub static PLAYER_DRAW: Static<Location> = Static::new(|| "player:draw".into());
pub static PLAYER_PLAY: Static<Location> = Static::new(|| "player:play".into());
pub static PLAYER_DAMAGE: Static<Location> = Static::new(|| "player:damage".into());
pub static ENEMY_DRAW: Static<Location> = Static::new(|| "enemy:draw".into());
pub static ENEMY_PLAY: Static<Location> = Static::new(|| "enemy:play".into());
pub static ENEMY_DAMAGE: Static<Location> = Static::new(|| "enemy:damage".into());

pub static MY_ATTRIBUTE_ASSIST: Static<TagKey> = Static::new(|| "$my:attribute:assist".into());
pub static ANY_RESOURCE_DAMAGE: Static<TagKey> = Static::new(|| "$any:damage:resource".into());
pub static ANY_ATTRIBUTE_RESIST: Static<TagKey> = Static::new(|| "$any:attribute:resist".into());
pub static ANY_RESOURCE: Static<TagKey> = Static::new(|| "$any:resource".into());

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
            ])
        };

        Self {
            states,
            cards: Cards::generate(),
            npcs: Npcs::generate(),
        }
    }
}

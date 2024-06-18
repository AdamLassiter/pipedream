use std::sync::LazyLock;

use crate::resource::{location::Location, tag::TagKey};

pub type Static<T> = LazyLock<T>;

pub static COMBAT_INIT: Static<Location> = Static::new(|| Location("combat:init".to_string()));
pub static PLAYER_DRAW: Static<Location> = Static::new(|| Location("player:draw".to_string()));
pub static PLAYER_PLAY: Static<Location> = Static::new(|| Location("player:play".to_string()));
pub static PLAYER_RESOLVE_PLAY: Static<Location> =
    Static::new(|| Location("player:play:resolve".to_string()));

pub static PLAYER_HAND: Static<TagKey> = Static::new(|| TagKey("player:hand".to_string()));
pub static PLAYER_DECK: Static<TagKey> = Static::new(|| TagKey("player:deck".to_string()));

pub static ENEMY_NAME: Static<TagKey> = Static::new(|| TagKey("enemy:name".to_string()));

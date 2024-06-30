use std::collections::BTreeMap;

use log::error;
use serde::{Deserialize, Serialize};

use crate::engine::core::tag::{Static, TagKey, Tags};

pub static PLAYER_NAME: Static<TagKey> = Static::new(|| "player:name".into());
pub static ENEMY_NAME: Static<TagKey> = Static::new(|| "enemy:name".into());

#[derive(Serialize, Deserialize)]
pub struct Npcs(pub BTreeMap<String, Npc>);

impl Npcs {
    pub fn find(&self, npc: &TagKey) -> &Npc {
        self.0.get(npc.trailing_key()).unwrap_or_else(|| {
            error!(target:"Combat/Lookup", "Failed to find npc by tag '{:?}'", npc.0);
            panic!("Failed to find npc by tag '{:?}'", npc.0)
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Npc {
    pub name: String,
    pub tags: Tags,
}

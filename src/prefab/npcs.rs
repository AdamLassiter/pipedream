use std::{collections::BTreeMap, fs::File};

use crate::resource::{
    combat::npc::{Npc, Npcs},
    core::tag::TagKey,
};

use super::tags::Static;

pub static PLAYER_NAME: Static<TagKey> = Static::new(|| TagKey("player:name".to_string()));
pub static ENEMY_NAME: Static<TagKey> = Static::new(|| TagKey("enemy:name".to_string()));

impl Npcs {
    fn dump(&self) {
        let buffer = File::create("./npcs.yml").unwrap();
        serde_yml::to_writer(buffer, &self).unwrap();
    }

    pub fn generate() -> Self {
        let cards = Self(BTreeMap::from_iter(
            Self::generate_vec()
                .into_iter()
                .map(|card| (card.name.clone(), card)),
        ));

        cards.dump();
        cards
    }

    fn generate_vec() -> Vec<Npc> {
        vec![
            Npc {
                name: "Slightly Larger Dave".into(),
                tags: vec!["$my:name:Slightly Larger Dave".into()].into(),
            },
            Npc {
                name: "Slightly Larger Dave".into(),
                tags: vec!["$my:name:Dave".into()].into(),
            },
        ]
    }
}

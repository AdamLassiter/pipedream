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
        let buffer =
            File::create("./npcs.yml").expect("Failed to open file for writing npcs data");
        serde_yml::to_writer(buffer, &self).expect("Failed to write yaml npcs data to file");
    }
    }

    pub fn generate() -> Self {
        let cards = Self(BTreeMap::from_iter(
            Self::generate_vec()
                .into_iter()
                .map(|npc| (npc.name.clone(), npc)),
        ));

        cards.dump();
        cards
    }

    fn generate_vec() -> Vec<Npc> {
        vec![
            Npc {
                name: "Slightly Larger Dave".into(),
                tags: vec!["enemy:name:Slightly Larger Dave".into()].into(),
            },
            Npc {
                name: "Dave".into(),
                tags: vec!["enemy:name:Dave".into()].into(),
            },
        ]
    }
}

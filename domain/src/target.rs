use rusqlite::Connection;
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize};

use crate::character::{Character, CharacterId};

use pipedream_engine::action::Action;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    Me,
    You,
}
#[derive(Clone, Debug)]
#[orm_autobind]
pub struct TargetCharacter {
    pub target: Target,
    pub character: CharacterId,
}
impl TargetCharacter {
    pub fn get_target_character(conn: &Connection, target: &Target) -> (CharacterId, Character) {
        let (
            _id,
            TargetCharacter {
                character: character_id,
                ..
            },
        ) = TargetCharacterDao::select_target(conn, target)
            .unwrap_or_else(|e| panic!("Failed to find Character for {:?}: {}", target, e))
            .pop()
            .unwrap_or_else(|| panic!("No Character found for {:?}", target))
            .into();
        let character = Character::get(conn, &character_id);
        (character_id, character)
    }

    pub fn update_target_character(
        conn: &Connection,
        target: &Target,
        update: impl Fn(Character) -> Character,
    ) -> Action {
        let (character_id, character) = Self::get_target_character(conn, target);
        let updated = update(character);
        updated.update_action(character_id)
    }
}

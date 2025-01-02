use rusqlite::Connection;
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize};

use crate::character::{Character, CharacterId};

use crate::action::Action;

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
        ) = Self::get_target(conn, target);
        let character = Character::get(conn, &character_id);
        (character_id, character)
    }

    pub fn insert_target_character(conn: &Connection, target: Self) {
        let target_dao: TargetCharacterDao = target.into();
        target_dao
            .insert(conn)
            .expect("Failed to insert TargetCharacter");
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

    pub fn delete_target_characters() -> Vec<Action> {
        vec![
            Action::pure(TargetCharacterDao::drop_table_sql()),
            Action::pure(TargetCharacterDao::create_table_sql()),
        ]
    }

    pub fn get_target(conn: &Connection, target: &Target) -> (TargetCharacterId, TargetCharacter) {
        let (id, target_char) = TargetCharacterDao::select_target(conn, target)
            .unwrap_or_else(|e| panic!("Failed to find Target for {:?}: {}", target, e))
            .pop()
            .unwrap_or_else(|| panic!("No Target found for {:?}", target))
            .into();
        (id.expect("No Id found for TargetCharacter"), target_char)
    }

    pub fn update_action(
        conn: &Connection,
        target: &Target,
        update: impl Fn(TargetCharacter) -> TargetCharacter,
    ) -> Action {
        let (target_char_id, target_char) = Self::get_target(conn, target);
        let updated = update(target_char);
        Action {
            sql: TargetCharacterDao::update_sql(&["target", "character"], &["id"]).to_string(),
            params: vec![
                (
                    ":id".to_string(),
                    serde_json::to_string(&target_char_id.0)
                        .expect("Failed to serialize Id to Json"),
                ),
                (
                    ":target".to_string(),
                    serde_json::to_string(&updated.target)
                        .expect("Failed to serialize Character Name to Json"),
                ),
                (
                    ":character".to_string(),
                    serde_json::to_string(&updated.character)
                        .expect("Failed to serialize Character Image to Json"),
                ),
            ],
        }
    }
}

use rusqlite::Connection;
use rusqlite_orm::orm_autobind;

use crate::player::Player;

use super::stats::Stats;
use pipedream_engine::{action::Action, image::Image};

#[derive(Clone, Debug)]
#[orm_autobind]
pub struct Character {
    pub name: String,
    pub image: Image,
    pub stats: Stats,
}

#[derive(Clone, Debug)]
#[orm_autobind]
pub struct PlayerCharacter {
    pub player: Player,
    pub character: CharacterId,
}

impl PlayerCharacter {
    pub fn get_player_character(conn: &Connection, player: &Player) -> (CharacterId, Character) {
        let (
            _id,
            PlayerCharacter {
                character: character_id,
                ..
            },
        ) = PlayerCharacterDao::select_player(conn, player)
            .ok()
            .and_then(|mut res| res.pop())
            .unwrap_or_else(|| panic!("Failed to find Character for {:?}", player))
            .into();
        let character = CharacterDao::select_id(conn, &character_id)
            .ok()
            .and_then(|mut res| res.pop())
            .unwrap_or_else(|| panic!("Failed to find Character for {:?}", character_id))
            .into();
        (character_id, character)
    }

    pub fn update_player_character(
        conn: &Connection,
        player: &Player,
        update: impl Fn(Character) -> Character,
    ) -> Action {
        let (character_id, character) = Self::get_player_character(conn, player);
        let updated = update(character);
        Action {
            sql_batch: vec![Character::update_sql().to_string()],
            params: vec![
                (
                    ":id".to_string(),
                    serde_json::to_value(character_id.0).expect("Failed to serialize Id to json"),
                ),
                (
                    ":data".to_string(),
                    serde_json::to_value(updated).expect("Failed to serialize Character to json"),
                ),
            ],
        }
    }
}

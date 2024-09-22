use rusqlite::Connection;
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize};

use crate::character::{Character, CharacterId};

use pipedream_engine::action::Action;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player {
    Human,
    Cpu,
    World,
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
            .unwrap_or_else(|e| panic!("Failed to find PlayerCharacter for {:?}: {}", player, e))
            .pop()
            .unwrap_or_else(|| panic!("No PlayerCharacter found for {:?}", player))
            .into();
        let character = Character::get(conn, &character_id);
        (character_id, character)
    }

    pub fn update_player_character(
        conn: &Connection,
        player: &Player,
        update: impl Fn(Character) -> Character,
    ) -> Action {
        let (character_id, character) = Self::get_player_character(conn, player);
        let updated = update(character);
        updated.update_action(character_id)
    }
}

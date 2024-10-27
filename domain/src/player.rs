use rusqlite::Connection;
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::character::{Character, CharacterId};

use crate::action::Action;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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
    pub fn find_player_character(
        conn: &Connection,
        player: &Player,
    ) -> Option<(CharacterId, Character)> {
        if let Some((
            _id,
            PlayerCharacter {
                character: character_id,
                ..
            },
        )) = PlayerCharacterDao::select_player(conn, player)
            .unwrap_or_else(|e| panic!("Failed to find PlayerCharacter for {:?}: {}", player, e))
            .pop()
            .map(|dao| dao.into())
        {
            let character = Character::get(conn, &character_id);

            Some((character_id, character))
        } else {
            None
        }
    }

    pub fn get_player_character(conn: &Connection, player: &Player) -> (CharacterId, Character) {
        Self::find_player_character(conn, player)
            .unwrap_or_else(|| panic!("No PlayerCharacter found for {:?}", player))
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

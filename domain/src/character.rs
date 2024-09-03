use rusqlite::Connection;
use rusqlite_orm::orm_bind;

use crate::encounter::{Player, PlayerCharacter};

use super::{card::Card, stats::Stats};
use pipedream_engine::{action::Action, image::Image, tag::Tag};

#[derive(Clone, Debug)]
#[orm_bind ({name: "$.name"}, [])]
pub struct Character {
    pub name: String,
    pub image: Image,
    pub tags: Vec<Tag>,
    pub deck: Vec<Card>,
    pub stats: Stats,
}

impl Character {
    pub fn get_player(conn: &Connection, player: &Player) -> Self {
        let (_id, PlayerCharacter { character, .. }) =
            PlayerCharacter::query_by_player(conn, player)
                .ok()
                .and_then(|mut res| res.pop())
                .unwrap_or_else(|| panic!("Failed to find EncounterCharacter for {:?}", player));
        character
    }

    pub fn update_player(
        conn: &Connection,
        player: &Player,
        update: impl Fn(Character) -> Self,
    ) -> Action {
        let (id, PlayerCharacter { character, .. }) =
            PlayerCharacter::query_by_player(conn, player)
                .ok()
                .and_then(|mut res| res.pop())
                .unwrap_or_else(|| panic!("Failed to find EncounterCharacter for {:?}", player));
        let updated = update(character);
        Action {
            sql_batch: vec![Character::update_sql().to_string()],
            params: vec![],
        }
    }
}

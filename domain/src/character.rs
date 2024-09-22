use rusqlite::Connection;
use rusqlite_orm::orm_autobind;

use super::stats::Stats;
use pipedream_engine::{action::Action, image::Image};

#[derive(Clone, Debug)]
#[orm_autobind]
pub struct Character {
    pub name: String,
    pub image: Image,
    pub stats: Stats,
}
impl Character {
    pub fn update_action(&self, character_id: CharacterId) -> Action {
        Action {
            sql: CharacterDao::update_sql(&["name", "image", "stats"], &["id"]).to_string(),
            params: vec![
                (
                    ":id".to_string(),
                    serde_json::to_string(&character_id.0).expect("Failed to serialize Id to Json"),
                ),
                (
                    ":name".to_string(),
                    serde_json::to_string(&&self.name)
                        .expect("Failed to serialize Character Name to Json"),
                ),
                (
                    ":image".to_string(),
                    serde_json::to_string(&&self.image)
                        .expect("Failed to serialize Character Image to Json"),
                ),
                (
                    ":stats".to_string(),
                    serde_json::to_string(&&self.stats)
                        .expect("Failed to serialize Character Stats to Json"),
                ),
            ],
        }
    }

    pub fn get(conn: &Connection, character_id: &CharacterId) -> Self {
        CharacterDao::select_id(conn, character_id)
            .unwrap_or_else(|e| panic!("Failed to find Character for {:?}: {}", character_id, e))
            .pop()
            .unwrap_or_else(|| panic!("No Character found for {:?}", character_id))
            .into()
    }
}

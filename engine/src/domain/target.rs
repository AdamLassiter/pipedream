use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Tgt {
    Any,
    Me,
    You,
    Player,
    Enemy,
}

impl Tgt {
    pub fn as_key(&self) -> &'static str {
        match self {
            Tgt::Any => "$any",
            Tgt::Me => "$me",
            Tgt::You => "$you",
            Tgt::Player => "player",
            Tgt::Enemy => "enemy",
        }
    }
}
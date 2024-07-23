use std::fmt::Display;

use crate::core::tags::TagKey;

use super::entity::Ent;

#[derive(Clone, Copy)]
pub enum Tgt {
    Any,
    Me,
    You,
    Player,
    Enemy,
}

impl From<Tgt> for String {
    fn from(val: Tgt) -> Self {
        match val {
            Tgt::Any => "$any",
            Tgt::Me => "$me",
            Tgt::You => "$you",
            Tgt::Player => "player",
            Tgt::Enemy => "enemy",
        }
        .to_string()
    }
}
impl Display for Tgt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: String = (*self).into();
        f.write_str(&this)
    }
}
impl Tgt {
    pub fn ent(self, entity: Ent) -> TagKey {
        let targeted: String = self.into();
        let entity: String = entity.into();
        TagKey(format!("{}:{}", targeted, entity))
    }
}

use std::fmt::Display;

use crate::core::tags::TagKey;

use super::target::Tgt;

#[derive(Clone, Copy)]
pub enum Ent {
    Name,
    Attribute,
    AttributeAssist,
    AttributeResist,
    Resource,
    ResourceHealth,
    Damage,
    Deck,
    Hand,
    DrawCount,
}

impl From<Ent> for String {
    fn from(val: Ent) -> Self {
        match val {
            Ent::Name => "name",
            Ent::Attribute => "attribute",
            Ent::AttributeAssist => "attribute:assist",
            Ent::AttributeResist => "attribute:resist",
            Ent::Resource => "resource",
            Ent::ResourceHealth => "resource:health",
            Ent::Damage => "damage",
            Ent::Deck => "deck",
            Ent::Hand => "hand",
            Ent::DrawCount => "draw:count",
        }
        .to_string()
    }
}

impl Display for Ent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: String = (*self).into();
        f.write_str(&this)
    }
}

impl Ent {
    pub fn tgt(self, target: Tgt) -> TagKey {
        let targeted: String = target.into();
        let entity: String = self.into();
        TagKey(format!("{}:{}", targeted, entity))
    }
}

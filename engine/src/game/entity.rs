use std::fmt::Display;

use crate::core::tags::TagKey;

use super::target::Tgt;

#[derive(Clone, Copy, Debug)]
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
    Item,
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
            Ent::Item => "item",
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
        target.ent(self)
    }

    pub fn pretty(&self) -> &'static str {
        match self {
            Self::Name => "Name",
            Self::Attribute => "Attributes",
            Self::AttributeAssist => "Assists",
            Self::AttributeResist => "Resists",
            Self::Resource => "Resources",
            Self::ResourceHealth => "Health",
            Self::Damage => "Damage",
            Self::Deck => "Deck",
            Self::Hand => "Hand",
            Self::DrawCount => "Draw Count",
            Self::Item => "Items",
        }
    }
}

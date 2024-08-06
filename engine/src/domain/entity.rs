use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl Ent {
    pub fn as_key(&self) -> &'static str {
        match self {
            Ent::Name => "name",
            Ent::Attribute => "attribute",
            Ent::AttributeAssist => "attribute:assist",
            Ent::AttributeResist => "attribute:resist",
            Ent::Resource => "resource",
            Ent::ResourceHealth => "resource:health",
            Ent::Damage => "damage",
            Ent::Deck => "deck",
            Ent::Hand => "hand",
            Ent::DrawCount => "draw-count",
            Ent::Item => "item",
        }
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

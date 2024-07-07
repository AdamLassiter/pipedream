use std::fmt::Display;

use log::debug;

use crate::{
    engine::{
        core::tag::{Static, TagKey, Tags},
        state::tag_engine::TagEngine,
    },
    prefab::tags::Tgt,
};

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

pub static ANY_SUBSTITUTIONS: Static<Vec<String>> =
    Static::new(|| vec![Tgt::Player.into(), Tgt::Enemy.into()]);

pub static FROM_CAMPAIGN: Static<Vec<TagKey>> = Static::new(|| {
    vec![
        Tgt::Any.ent(Ent::Name),
        Tgt::Any.ent(Ent::Attribute),
        Tgt::Any.ent(Ent::Resource),
        Tgt::Any.ent(Ent::Deck),
        Tgt::Any.ent(Ent::DrawCount),
    ]
});

impl TagEngine {
    pub fn generate_campaign() -> Self {
        Self {
            tags: Tags::from(vec![
                // World
                "woods:entrance:item:sword".into(),
                // Player
                "$player:draw:count/2".into(),
                // Resources
                "$player:resource:health/20".into(),
                "$player:resource:stamina/20".into(),
                "$player:resource:mana/20".into(),
                "$player:resource:faith/20".into(),
                // Deck
                "$player:deck:Anathema Device".into(),
                "$player:deck:Bag of Endless Bags".into(),
                "$player:deck:Regular Punch".into(),
                "$player:deck:Consecutive Regular Punches".into(),
            ]),
        }
    }

    pub fn into_combat(campaign_tags: &Self) -> Self {
        let from_campaign = FROM_CAMPAIGN
            .iter()
            .flat_map(|from_campaign| campaign_tags.find(from_campaign))
            .collect::<Vec<_>>();

        debug!(target:"Event/IntoCombat", "{:?}", from_campaign);
        Self {
            tags: from_campaign.into(),
        }
    }
}

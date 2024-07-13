use log::debug;
use pipedream_engine::{
    combat::{entity::Ent, target::Tgt},
    core::tags::{Static, TagKey, Tags},
    state::tag_engine::TagEngine,
};

use crate::{Buildable, Generatable};

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
pub static FROM_COMBAT: Static<Vec<TagKey>> = Static::new(|| {
    vec![
        Tgt::Any.ent(Ent::Name),
        Tgt::Any.ent(Ent::Attribute),
        Tgt::Any.ent(Ent::Resource),
        Tgt::Any.ent(Ent::Deck),
        Tgt::Any.ent(Ent::DrawCount),
    ]
});

impl Generatable for TagEngine {
    fn generate() -> Self {
        Self {
            tags: Tags::build(vec![
                // World
                "woods:entrance:item:sword".into(),
                // Player
                "player:draw:count/4".into(),
                // Resources
                "player:resource:health/20".into(),
                "player:resource:stamina/20".into(),
                "player:resource:mana/20".into(),
                "player:resource:faith/20".into(),
                // Deck
                "player:deck:Anathema Device".into(),
                "player:deck:Bag of Endless Bags".into(),
                "player:deck:Regular Punch/3".into(),
                "player:deck:Immolate".into(),
            ]),
        }
    }
}

pub fn into_combat(campaign_tags: &TagEngine) -> TagEngine {
    let from_campaign = FROM_CAMPAIGN
        .iter()
        .flat_map(|from_campaign| campaign_tags.find(from_campaign))
        .collect::<Vec<_>>();

    debug!(target:"State/IntoCombat", "{:?}", from_campaign);
    TagEngine {
        tags: Tags::build(from_campaign),
    }
}

pub fn from_combat(combat_tags: &TagEngine) -> TagEngine {
    let from_combat = FROM_COMBAT
        .iter()
        .flat_map(|from_combat| combat_tags.find(from_combat))
        .collect::<Vec<_>>();

    debug!(target:"State/FromCombat", "{:?}", from_combat);
    TagEngine {
        tags: Tags::build(from_combat),
    }
}

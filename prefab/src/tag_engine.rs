use pipedream_engine::{
    core::tag::{Static, TagKey, Tags},
    domain::{entity::Ent, target::Target},
    log::debug,
    state::tag_engine::TagEngine,
};

use crate::{npcs::generate_player, Buildable, Generatable};

pub static ANY_SUBSTITUTIONS: Static<Vec<String>> =
    Static::new(|| vec![Target::Player.into(), Target::Enemy.into()]);

pub static FROM_CAMPAIGN: Static<Vec<TagKey>> = Static::new(|| {
    vec![
        Target::Any.ent(Ent::Name),
        Target::Any.ent(Ent::Attribute),
        Target::Any.ent(Ent::Resource),
        Target::Any.ent(Ent::Deck),
        Target::Any.ent(Ent::DrawCount),
    ]
});
pub static FROM_COMBAT: Static<Vec<TagKey>> = Static::new(|| {
    vec![
        Target::Any.ent(Ent::Name),
        Target::Any.ent(Ent::Attribute),
        Target::Any.ent(Ent::Resource),
        Target::Any.ent(Ent::Deck),
        Target::Any.ent(Ent::DrawCount),
    ]
});

impl Generatable for TagEngine {
    fn generate() -> Self {
        Self {
            tags: Tags::build(
                vec![
                    // World
                    "woods:entrance:item:sword".into(),
                ]
                .into_iter()
                .chain(
                    // Player
                    generate_player().0.tags.iter_tags(),
                )
                .collect::<Vec<_>>(),
            ),
        }
    }
}

pub fn into_combat(campaign_tags: &TagEngine) -> TagEngine {
    let from_campaign = FROM_CAMPAIGN
        .iter()
        .flat_map(|from_campaign| campaign_tags.find(from_campaign))
        .collect::<Vec<_>>();

    debug!(target:"Prefab/TagEngine/IntoCombat", "{:?}", from_campaign);
    TagEngine {
        tags: Tags::build(from_campaign),
    }
}

pub fn from_combat(combat_tags: &TagEngine) -> TagEngine {
    let from_combat = FROM_COMBAT
        .iter()
        .flat_map(|from_combat| combat_tags.find(from_combat))
        .collect::<Vec<_>>();

    debug!(target:"Prefab/TagEngine/FromCombat", "{:?}", from_combat);
    TagEngine {
        tags: Tags::build(from_combat),
    }
}

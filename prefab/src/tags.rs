use std::collections::BTreeMap;

use pipedream_engine::{
    combat::target::Tgt,
    core::tags::{Static, Tag, TagKey, Tags},
};

use crate::Buildable;

pub static ME_REF: Static<TagKey> = Static::new(|| "combatant:reference:me".into());
pub static YOU_REF: Static<TagKey> = Static::new(|| "combatant:reference:you".into());
pub static PANIC_CONCRETE_ANY: Static<TagKey> = Static::new(|| "panic:concrete:any".into());

pub static SUBSTITUTIONS: Static<Vec<(String, TagKey)>> = Static::new(|| {
    vec![
        (Tgt::Me.into(), ME_REF.clone()),
        (Tgt::You.into(), YOU_REF.clone()),
        (Tgt::Any.into(), PANIC_CONCRETE_ANY.clone()),
    ]
});

pub static VARIANTS: Static<Vec<(String, Vec<String>)>> =
    Static::new(|| vec![(Tgt::Any.into(), vec![Tgt::Player.into(), Tgt::Enemy.into()])]);

impl Buildable<Tag> for Tags {
    fn build(tags: Vec<Tag>) -> Self {
        let kv_pairs = tags.iter().map(|tag| tag.into());
        Tags::new(
            BTreeMap::from_iter(kv_pairs),
            SUBSTITUTIONS.clone(),
            VARIANTS.clone(),
        )
    }
}

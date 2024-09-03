use std::collections::BTreeMap;

use pipedream_engine::{
    domain::target::Target,
    tag::{Static, Tag, TagKey, Tags},
};

use crate::Buildable;

pub static ME_REF: Static<TagKey> = Static::new(|| "combatant:reference:me".into());
pub static YOU_REF: Static<TagKey> = Static::new(|| "combatant:reference:you".into());
pub static PANIC_CONCRETE_ANY: Static<TagKey> = Static::new(|| "panic:concrete:any".into());

pub static SUBSTITUTIONS: Static<Vec<(String, TagKey)>> = Static::new(|| {
    vec![
        (Target::Me.into(), ME_REF.clone()),
        (Target::You.into(), YOU_REF.clone()),
        (Target::Any.into(), PANIC_CONCRETE_ANY.clone()),
    ]
});

pub static VARIANTS: Static<Vec<(String, Vec<String>)>> = Static::new(|| {
    vec![(
        Target::Any.into(),
        vec![Target::Player.into(), Target::Enemy.into()],
    )]
});

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

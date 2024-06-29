use std::{
    clone::Clone,
    collections::{btree_map::Range, BTreeMap},
    ops::Deref,
    str::FromStr,
};

use fixed::{types::extra::U16, FixedI64};
use serde::{Deserialize, Serialize};

use crate::{prefab::tags::Static, resource::combat::field::Combatant};

pub type FI64 = FixedI64<U16>;

pub static KEY_SEP: char = ':';
pub static VAL_SEP: char = '/';

pub static SUBSTITUTIONS: Static<BTreeMap<String, TagKey>> = Static::new(|| {
    BTreeMap::from_iter([
        ("$me".into(), "combatant:ref:me".into()),
        ("$you".into(), "combatant:ref:you".into()),
    ])
});

static MAX_RESOLVE_DEPTH: usize = 256;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag {
    pub key: TagKey,
    pub value: TagValue,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TagKey(pub String);

impl Deref for TagKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TagKey {
    pub fn trailing_key(&self) -> &str {
        self.split(KEY_SEP).last().unwrap_or("")
    }

    pub fn targets(&self) -> (Combatant, TagKey) {
        let mut split = self.split(KEY_SEP);
        let combatant = split.next().expect("Failed to parse targeted TagKey into combatatnt");
        let remainder = split.remainder().expect("Failed to parse targeted TagKey into remainder");

        (
            Combatant::from_str(combatant).expect("Failed to parse targeted combatant"),
            TagKey(remainder.into()),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TagValue {
    Tag(TagKey),
    Number(FI64),
}

impl From<TagValue> for String {
    fn from(val: TagValue) -> Self {
        match val {
            TagValue::Tag(s) => s.0,
            TagValue::Number(n) => format!("{}", n),
        }
    }
}

impl From<&str> for TagKey {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<&str> for TagValue {
    fn from(value: &str) -> Self {
        value
            .parse::<FI64>()
            .map_or(Self::Tag(value.into()), Self::Number)
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        let parts = value.split(VAL_SEP).collect::<Vec<_>>();
        let (&key, val) = (parts.first().expect("Empty tag"), parts.get(1));

        Self {
            key: key.into(),
            value: val
                .map(|&v| TagValue::from(v))
                .unwrap_or(TagValue::Number(1.into())),
        }
    }
}

impl From<String> for Tag {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<(TagKey, TagValue)> for Tag {
    fn from((key, value): (TagKey, TagValue)) -> Self {
        Self { key, value }
    }
}

impl From<&Tag> for (TagKey, TagValue) {
    fn from(value: &Tag) -> Self {
        let Tag { key, value } = value;
        (key.clone(), value.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags(pub BTreeMap<TagKey, TagValue>);

impl From<Vec<Tag>> for Tags {
    fn from(tags: Vec<Tag>) -> Self {
        let kv_pairs = tags.iter().map(|tag| tag.into());
        Tags(BTreeMap::from_iter(kv_pairs))
    }
}

impl Tags {
    pub fn insert(&mut self, key: &TagKey, value: &TagValue) -> Option<FI64> {
        match self
            .0
            .insert(self.resolve_key(key), self.resolve_value(value))
        {
            Some(insert) => match insert {
                TagValue::Tag(_) => panic!("Insertion returned Tag even after resolve to Number"),
                TagValue::Number(val) => Some(val),
            },
            None => None,
        }
    }

    pub fn remove(&mut self, key: &TagKey) -> Option<TagValue> {
        self.0.remove(key)
    }

    pub fn get(&self, key: &TagKey) -> Option<&TagValue> {
        self.0.get(key)
    }

    pub fn range(
        &self,
        bounds: (std::ops::Bound<TagKey>, std::ops::Bound<TagKey>),
    ) -> Range<'_, TagKey, TagValue> {
        self.0.range(bounds)
    }

    fn resolve_key(&self, key: &TagKey) -> TagKey {
        let mut key = key.0.clone();
        SUBSTITUTIONS.iter().for_each(|(target, reference)| {
            if key.contains(target) {
                let substitution = match self.0.get(reference).expect("Failed to resolve reference for key") {
                    TagValue::Tag(key) => key,
                    TagValue::Number(value) => {
                        panic!(
                            "Expected Tag reference when resolving key {:?}, but was Number {:?}",
                            key, value
                        )
                    }
                };
                key = key.replace(target, substitution);
            }
        });
        TagKey(key)
    }

    fn resolve_value(&self, value: &TagValue) -> TagValue {
        let mut next = value.clone();
        for _ in 1..MAX_RESOLVE_DEPTH {
            match next {
                TagValue::Tag(tk) => match self.0.get(&tk).expect("Failed to resolve reference for value").clone() {
                    TagValue::Number(val) => {
                        next = TagValue::Number(val);
                    }
                    TagValue::Tag(tag) => return TagValue::Tag(tag),
                },
                TagValue::Number(val) => return TagValue::Number(val),
            }
        }
        panic!(
            "Expected to resolve value {:?} but reached max depth at {:?}",
            value, next
        );
    }
}

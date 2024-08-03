use std::{
    clone::Clone, collections::{BTreeMap, VecDeque}, ops::Bound::Included, str::FromStr
};

use fixed::{types::extra::U16, FixedI64};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::game::field::Combatant;

use std::sync::LazyLock;

pub type Static<T> = LazyLock<T>;

pub type FI64 = FixedI64<U16>;

pub static KEY_SEP: char = ':';
pub static VAL_SEP: char = '=';

static MAX_RESOLVE_DEPTH: usize = 256;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag {
    pub key: TagKey,
    pub value: TagValue,
}

pub static TAG_STYLES: Static<BTreeMap<&str, &str>> = Static::new(|| {
    BTreeMap::from_iter([
        ("health", "red"),
        ("stamina", "green"),
        ("mana", "blue"),
        ("faith", "yellow"),
    ])
});
impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style = TAG_STYLES
            .get(self.key.trailing_key())
            .copied()
            .unwrap_or("x");
        f.write_str(format!("<{} {}={}>", style, self.key.trailing_key(), self.value).as_str())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TagKey(pub String);

impl TagKey {
    pub fn trailing_key(&self) -> &str {
        self.0.split(KEY_SEP).last().unwrap_or("")
    }

    pub fn trailing_subpath(&self) -> Self {
        let mut subpath = self.0.split(KEY_SEP).collect::<Vec<_>>();
        subpath.pop();
        Self(subpath.join(&KEY_SEP.to_string()))
    }

    pub fn leading_key(&self) -> &str {
        self.0.split(KEY_SEP).next().unwrap_or("")
    }

    pub fn leading_subpath(&self) -> Self {
        let mut subpath = self.0.split(KEY_SEP).collect::<VecDeque<_>>();
        subpath.pop_front();
        Self(Vec::from(subpath).join(&KEY_SEP.to_string()))
    }

    pub fn targets(&self) -> (Combatant, TagKey) {
        let mut split = self.0.split(KEY_SEP);
        let combatant = split
            .next()
            .expect("Failed to parse targeted TagKey into combatatnt");
        let remainder = split
            .remainder()
            .expect("Failed to parse targeted TagKey into remainder");

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

impl TagValue {
    pub fn tag(&self) -> Option<&TagKey> {
        match self {
            Self::Tag(key) => Some(key),
            Self::Number(_) => None,
        }
    }

    pub fn number(&self) -> Option<&FI64> {
        match self {
            Self::Tag(_) => None,
            Self::Number(num) => Some(num),
        }
    }
}

impl std::fmt::Display for TagValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Self::Tag(key) => key.trailing_key().to_string(),
                Self::Number(num) => format!("{}", num),
            }
            .as_str(),
        )
    }
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
pub struct Tags {
    tags: BTreeMap<TagKey, TagValue>,
    variants: Vec<(String, Vec<String>)>,
    substitutions: Vec<(String, TagKey)>,
}

impl Tags {
    pub fn new(
        tags: BTreeMap<TagKey, TagValue>,
        substitutions: Vec<(String, TagKey)>,
        variants: Vec<(String, Vec<String>)>,
    ) -> Self {
        Self {
            tags,
            substitutions,
            variants,
        }
    }

    pub fn get_variants(&self, tag_key: &TagKey) -> Vec<TagKey> {
        self.variants
            .iter()
            .flat_map(|(variable, variants)| {
                if tag_key.leading_key() == variable {
                    variants
                        .iter()
                        .map(|variant| TagKey(tag_key.0.replace(variable, variant)))
                        .collect()
                } else {
                    vec![tag_key.clone()]
                }
                .into_iter()
            })
            .collect::<Vec<_>>()
    }

    pub fn insert(&mut self, key: &TagKey, value: &TagValue) {
        if let TagValue::Number(num) = value
            && num.is_zero()
        {
            self.remove(key);
        } else {
            self.tags
                .insert(self.concrete_key(key), self.resolve_value(value));
            debug!(target:"Engine/Tags/Insert", "{:?} {:?}", key, value);
        }
    }

    pub fn remove(&mut self, key: &TagKey) {
        self.get_variants(key).iter().for_each(|key| {
            self.tags.remove(&self.concrete_key(key));
        });

        debug!(target:"Engine/Tags/Remove", "{:?}", key);
    }

    pub fn get(&self, key: &TagKey) -> Option<&TagValue> {
        let value = self
            .get_variants(key)
            .iter()
            .filter_map(|key| {
                let key = &self.concrete_key(key);
                self.tags.get(key)
            })
            .next();

        debug!(target:"Engine/Tags/Get", "{:?} {:?}", key, value);
        value
    }

    pub fn find(&self, partial_key: &TagKey) -> Vec<Tag> {
        let found = self
            .get_variants(partial_key)
            .iter()
            .flat_map(|partial_key| {
                let partial_key = self.concrete_key(partial_key);

                let start = Included(partial_key.clone());
                let mut end_str = partial_key.clone().0;
                end_str.push('~');
                let end = Included(end_str.as_str().into());
                self.tags
                    .range((start, end))
                    .map(|(k, v)| Tag::from((k.clone(), v.clone())))
            })
            .collect();

        debug!(target:"Engine/Tags/Find", "{:?} -> {:?}", partial_key, found);
        found
    }

    fn concrete_key(&self, key: &TagKey) -> TagKey {
        let mut key = key.0.clone();
        self.substitutions.clone()
            .iter()
            .for_each(|(target, reference)| {
                if key.contains(target) {
                    let substitution = self.tags.get(reference).unwrap_or_else(|| {
                        panic!(
                            "Failed to resolve reference {:?} for target {:?} and key {:?} and self {:?}",
                            reference, target, key, self.tags,
                        )
                    }).tag().unwrap_or_else(|| {
                            panic!(
                                "Expected Tag reference when resolving key {:?}, but was Number",
                                key)
                        });
                    debug!(target:"Engine/Tags/Resolve", "{:?} {:?} {:?}", key, target, substitution);
                    key = key.replace(target, &substitution.0);
                }
            });
        TagKey(key)
    }

    fn resolve_value(&self, value: &TagValue) -> TagValue {
        let mut next = value.clone();
        for _ in 1..MAX_RESOLVE_DEPTH {
            match next {
                TagValue::Tag(ref tk) => match self.tags.get(tk).cloned() {
                    Some(TagValue::Number(val)) => {
                        next = TagValue::Number(val);
                    }
                    Some(TagValue::Tag(tag)) => return TagValue::Tag(tag),
                    None => return next,
                },
                TagValue::Number(val) => return TagValue::Number(val),
            }
        }
        panic!(
            "Expected to resolve value {:?} but reached max depth at {:?}",
            value, next
        );
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<TagKey, TagValue> {
        self.tags.iter()
    }
}

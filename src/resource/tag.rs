use std::{clone::Clone, collections::BTreeMap, ops::Deref};

use serde::{Deserialize, Serialize};

pub static VAL_SEP: char = '/';

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag(pub TagKey, pub TagValue);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TagKey(pub String);

impl Deref for TagKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagValue {
    Tag(TagKey),
    Number(f64),
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
            .parse::<f64>()
            .map_or(Self::Tag(value.into()), Self::Number)
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        let parts = value.split(VAL_SEP).collect::<Vec<_>>();
        let (&key, val) = (parts.first().unwrap(), parts.get(1));

        Self(
            key.into(),
            val.map(|&v| TagValue::from(v))
                .unwrap_or(TagValue::Number(1.)),
        )
    }
}

impl From<String> for Tag {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<(TagKey, TagValue)> for Tag {
    fn from((key, value): (TagKey, TagValue)) -> Self {
        Self(key, value)
    }
}

impl From<&Tag> for (TagKey, TagValue) {
    fn from(val: &Tag) -> Self {
        let Tag(key, val) = val;
        (key.clone(), val.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags(pub BTreeMap<TagKey, TagValue>);

impl<const N: usize> From<[Tag; N]> for Tags {
    fn from(tags: [Tag; N]) -> Self {
        let kv_pairs = tags.iter().map(|tag| tag.into());
        Tags(BTreeMap::from_iter(kv_pairs))
    }
}

impl std::ops::DerefMut for Tags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Tags {
    type Target = BTreeMap<TagKey, TagValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

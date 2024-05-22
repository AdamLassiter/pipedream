use std::{collections::BTreeMap, ops::Deref};

use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{List, Widget},
};
use serde::{Deserialize, Serialize};

pub static VAL_SEP: char = '/';

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag(pub TagKey, pub TagValue);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TagKey(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagValue {
    Tag(TagKey),
    Number(f64),
}

impl Into<String> for TagValue {
    fn into(self) -> String {
        match self {
            Self::Tag(s) => s.0,
            Self::Number(n) => format!("{}", n),
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
            .map_or(Self::Tag(value.into()), |num| Self::Number(num))
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        let parts = value.split(VAL_SEP).collect::<Vec<_>>();
        let (&key, val) = (parts.get(0).unwrap(), parts.get(1));
        let normalised = Self(
            key.into(),
            val.map(|&v| TagValue::from(v))
                .unwrap_or(TagValue::Number(1.))
                .into(),
        );
        normalised
    }
}

impl Into<(TagKey, TagValue)> for &Tag {
    fn into(self) -> (TagKey, TagValue) {
        let Tag(key, val) = self;
        (key.clone(), val.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags(BTreeMap<TagKey, TagValue>);

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

impl Widget for &Tags {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(
            List::new(
                self.iter()
                    .map(|(key, val)| format!("{:?}/{:?}", key.0, val)),
            ),
            area,
            buf,
        );
    }
}

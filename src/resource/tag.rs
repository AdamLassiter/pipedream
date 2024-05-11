use std::{collections::BTreeSet, ops::Deref};

use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{List, Widget},
};
use serde::{Deserialize, Serialize};

static TAG_SEP: &str = ":";

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Tag(String);

pub enum TagType {
    String,
    Number,
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Tag(value.into())
    }
}

impl Tag {
    pub fn typehint(&self) -> TagType {
        match self.parts().last().unwrap().parse::<f64>() {
            Ok(_) => TagType::Number,
            Err(_) => TagType::String,
        }
    }

    pub fn pretty(&self) -> String {
        self.parts().join(TAG_SEP)
    }

    pub fn parts(&self) -> Vec<String> {
        self.0.split(TAG_SEP).map(|s| s.into()).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags(pub BTreeSet<Tag>);

impl std::ops::DerefMut for Tags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Tags {
    type Target = BTreeSet<Tag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Widget for &Tags {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(List::new(self.iter().map(|tag| tag.pretty())), area, buf);
    }
}

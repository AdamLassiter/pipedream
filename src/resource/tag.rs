use std::{collections::BTreeSet, ops::Deref};

use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{List, Widget},
};
use serde::{Deserialize, Serialize};

pub static TAG_SEP: char = ':';

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Tag(String);

#[derive(Eq, PartialEq)]
pub enum TypeHint {
    String,
    Number,
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Tag(value.into())
    }
}

impl From<Vec<String>> for Tag {
    fn from(value: Vec<String>) -> Self {
        Tag(value.join(&TAG_SEP.to_string()))
    }
}

impl Tag {
    pub fn tag_sep() -> &'static char {
        &TAG_SEP
    }

    pub fn typehint(&self) -> TypeHint {
        if let Ok(_) = self.parts().last().unwrap().parse::<f64>() {
            TypeHint::Number
        } else {
            TypeHint::String
        }
    }

    pub fn pop(&self) -> Self {
        let mut parts = self.parts();
        parts.pop();
        Tag::from(parts)
    }

    pub fn append(&self, mut tail: Vec<String>) -> Self {
        let mut parts = self.parts();
        parts.append(&mut tail);
        Tag::from(parts)
    }

    pub fn pretty(&self) -> String {
        self.parts().join(&TAG_SEP.to_string())
    }

    pub fn parts(&self) -> Vec<String> {
        self.0.split(TAG_SEP).map(|s| s.into()).collect()
    }

    pub fn wildcarding_numbers(&self) -> Self {
        match self.typehint() {
            TypeHint::Number => self.pop(),
            TypeHint::String => self.clone(),
        }
    }

    pub fn as_number(&self) -> f64 {
        match self.typehint() {
            TypeHint::Number => {
                let mut parts = self.parts();
                parts.pop().unwrap().parse::<f64>().unwrap()
            }
            TypeHint::String => 1.,
        }
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

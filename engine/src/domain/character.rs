use rusqlite_orm::orm_bind;
use serde::{Deserialize, Serialize};

use super::{card::Card, stats::Stats};
use crate::core::{image::Image, tag::Tag};

#[derive(Clone, Debug)]
#[orm_bind ({name: "$.name"}, [])]
pub struct Character {
    pub name: String,
    pub image: Image,
    pub tags: Vec<Tag>,
    pub deck: Vec<Card>,
    pub stats: Stats,
}

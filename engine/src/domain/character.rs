use rusqlite_orm::orm_bind;
use serde::{Deserialize, Serialize};

use super::{card::Card, stats::{EphemeralStats, Stats}};
use crate::core::{image::Image, tag::Tag};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[orm_bind {name: "$.name"}]
pub struct Character {
    pub name: String,
    pub image: Image,
    pub tags: Vec<Tag>,
    pub deck: Vec<Card>,
    pub stats: Stats,
    pub ephemeral_stats: Option<EphemeralStats>,
}

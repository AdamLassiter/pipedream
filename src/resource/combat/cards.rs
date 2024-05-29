use std::collections::BTreeMap;

use serde::Serialize;

use crate::resource::tag::Tag;

use super::{
    field::CombatPlace,
    stats::{Condition, Element, Resource},
};

#[derive(Serialize)]
pub struct Cards(BTreeMap<CombatPlace, Vec<Card>>);

#[derive(Serialize)]
pub struct Card {
    pub costs: Vec<(Resource, i64)>,
    pub damages: Vec<(Element, i64)>,
    pub conditions: Vec<(Condition, i64)>,
    pub manipulations: Vec<(CombatPlace, CombatPlace, u64)>,
    pub specials: Vec<Tag>,
    pub tags: Vec<Tag>,
}

#[derive(Serialize)]
pub struct Decklist(Class);
#[derive(Serialize)]
pub enum Class {
    Barbarian,
    Monk,
    Rogue,
    Mage,
}

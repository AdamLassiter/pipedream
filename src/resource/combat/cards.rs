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
    pub applies_tags: Vec<Tag>,
    pub has_tags: Vec<Tag>,
}

// #[derive(Serialize)]
// pub struct Decklist(Class);

// #[derive(Serialize)]
// pub enum Class {
//     Barbarian,
//     Warrior,
//     Beserker,
//     Tranced,

//     Monk,
//     Priest,
//     Cleric,
//     Chosen,

//     Rogue,
//     Assassin,
//     Bladedancer,
//     Soulknife,

//     Mage,
//     Archmage,
//     Wizard,
//     Weaver,
// }

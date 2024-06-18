use serde::{Deserialize, Serialize};

use super::{card::Cards, stats::Stats};

#[derive(Serialize, Deserialize)]
pub struct CombatEntity {
    pub cards: Cards,
    pub stats: Stats,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CombatSide {
    Mine,
    Yours,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FieldPlace {
    Innate,
    Deck,
    Hand,
    Discard,
    Vanish,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CombatPlace {
    pub side: CombatSide,
    pub place: FieldPlace,
}

use serde::Serialize;

use super::{cards::Cards, stats::Stats};

#[derive(Serialize)]
pub struct CombatEntity {
    pub(crate) cards: Cards,
    pub(crate) stats: Stats,
}

#[derive(Serialize)]
pub enum CombatPlace {
    Innate,
    Deck,
    Hand,
    Discard,
    Vanish,
}

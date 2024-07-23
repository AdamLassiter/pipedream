use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumString)]
pub enum Combatant {
    Me,
    You,
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
    pub side: Combatant,
    pub place: FieldPlace,
}

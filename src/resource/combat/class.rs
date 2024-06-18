use serde::{Deserialize, Serialize};

use super::card::Card;

#[derive(Serialize, Deserialize)]
pub struct Decklist {
    pub class: Class,
    pub deck: Vec<Card>,
}

#[derive(Serialize, Deserialize)]
pub enum Class {
    Barbarian,
    Warrior,
    Beserker,
    Tranced,

    Monk,
    Priest,
    Cleric,
    Chosen,

    Rogue,
    Assassin,
    Bladedancer,
    Soulknife,

    Mage,
    Archmage,
    Wizard,
    Weaver,
}

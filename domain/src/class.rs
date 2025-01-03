use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

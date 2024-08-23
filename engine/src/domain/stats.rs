use std::collections::BTreeMap;

use rusqlite_orm::orm_bind;
use serde::{Deserialize, Serialize};

use super::encounter::Player;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub resources: BTreeMap<Resource, i64>,
    pub max_resources: BTreeMap<Resource, i64>,
    pub sleight_of_hand: BTreeMap<SleightOfHand, i16>,
    pub assisstances: BTreeMap<Assistance, i16>,
    pub resistances: BTreeMap<Resistance, i16>,
    pub buffs: BTreeMap<Buff, i16>,
    pub debuffs: BTreeMap<Debuff, i16>,
}

#[derive(Clone, Debug)]
#[orm_bind ({ source: "$.source", target: "$.target", stat: "$.stat" }, [])]
pub struct StatChange {
    pub source: Player,
    pub target: Player,
    pub stat: Stat,
    pub change: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stat {
    SleightOfHand(SleightOfHand),
    Element(Element),
    Assistance(Assistance),
    Resistance(Resistance),
    Buff(Buff),
    Debuff(Debuff),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resource {
    Health,
    Stamina,
    Mana,
    Favour,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SleightOfHand {
    Inspiration,  // Draw count
    Versatility,  // Hand size
    Tranquility,  // Deck min
    Recollection, // Deck max
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Assistance {
    Strength,
    Dexterity,
    Intelligence,
    Faith,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resistance {
    Endurance,
    Vitality,
    Adaptibility,
    Fortitude,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Element {
    // Physical
    Bludgeoning,
    Piercing,
    Slashing,
    Force,

    // Poisons
    Toxic,
    Lethargy,
    Bloodhex,
    Manaburn,

    // Magic
    Cold,
    Acid,
    Lightning,
    Fire,

    // Divine
    Radiant,
    Psychic,
    Vampiric,
    Necrotic,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Buff {
    Overwhelm,
    Guard,
    Crush,
    Endure,

    Stab,
    Sneak,
    Dose,
    Dope,

    Evoke,
    Abjure,
    Invoke,
    Transmute,

    Hallow,
    Divine,
    Bless,
    Protect,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Debuff {
    Stun,
    Bleed,
    Disarm,
    Shatter,

    Poison,
    Exhaust,
    Curse,
    Unravel,

    Frostbite,
    Corrode,
    Shock,
    Burn,

    Hollow,
    Madden,
    Wilt,
    Rot,
}

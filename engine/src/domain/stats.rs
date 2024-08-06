use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub resources: BTreeMap<Resource, f64>,
    pub max_resources: BTreeMap<Resource, f64>,
    pub assisstances: BTreeMap<Assistance, f64>,
    pub resistances: BTreeMap<Resistance, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EphemeralStats {
    pub buffs: BTreeMap<Buff, f64>,
    pub debuffs: BTreeMap<Debuff, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resource {
    Health,
    Stamina,
    Mana,
    Favour,
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
    Bludgeoning,
    Piercing,
    Slashing,
    Force,

    Toxic,
    Lethargy,
    Bloodhex,
    Manaburn,

    Cold,
    Acid,
    Lightning,
    Fire,

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
    Unfalter,

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

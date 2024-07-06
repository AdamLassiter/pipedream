use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Resource {
    Health,
    Stamina,
    Mana,
    Favour,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Attribute {
    Assistance(Assistance),
    Resistance(Resistance),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Assistance {
    Strength,
    Dexterity,
    Intelligence,
    Faith,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Resistance {
    Endurance,
    Vitality,
    Adaptibility,
    Fortitude,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Condition {
    Buff(Buff),
    Debuff(Debuff),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

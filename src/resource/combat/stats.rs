use std::collections::BTreeMap;

use serde::Serialize;

use crate::engine::tag_engine::TagEngine;

#[derive(Serialize)]
pub struct Stats {
    pub(crate) resources: Resources,
    pub(crate) attributes: Attributes,
    pub(crate) tags: TagEngine,
}

#[derive(Serialize)]
pub struct Resources(BTreeMap<Resource, i64>);

#[derive(Serialize)]
pub enum Resource {
    Health,
    Stamina,
    Mana,
    Favour,
}

#[derive(Serialize)]
pub struct Attributes(BTreeMap<Attribute, i64>);

#[derive(Serialize)]
pub enum Attribute {
    Strength,
    Endurance,

    Dexterity,
    Vitality,

    Intelligence,
    Adaptibility,

    Faith,
    Fortitude,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub enum Condition {
    Buff(Buff),
    Debuff(Debuff),
}

#[derive(Serialize)]
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

#[derive(Serialize)]
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

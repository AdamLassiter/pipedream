use std::{collections::BTreeMap, fs::File};

use serde::Serialize;

use crate::{
    engine::tag_engine::TagEngine,
    resource::{prefab::combat_world::CombatWorld, tag::Tag},
};

use super::dynamic_state_machine::DynamicStateMachine;

#[derive(Serialize)]
pub struct Combat {
    pub tag_engine: TagEngine,
    pub player: CombatEntity,
    pub enemy: CombatEntity,
    #[serde(skip_serializing)]
    pub state_machine: DynamicStateMachine<CombatWorld>,
}

#[derive(Serialize)]
pub struct CombatEntity {
    cards: Cards,
    stats: Stats,
}

#[derive(Serialize)]
pub struct Cards(BTreeMap<CombatPlace, Vec<Card>>);

#[derive(Serialize)]
pub enum CombatPlace {
    Innate,
    Deck,
    Hand,
    Discard,
    Vanish,
}

#[derive(Serialize)]
pub struct Card {
    costs: Vec<(Resource, i64)>,
    damages: Vec<(Element, i64)>,
    conditions: Vec<(Condition, i64)>,
    manipulations: Vec<(CombatPlace, CombatPlace, u64)>,
    specials: Vec<Tag>,
    tags: Vec<Tag>,
}

#[derive(Serialize)]
pub struct Stats {
    resources: Resources,
    attributes: Attributes,
    tags: TagEngine,
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

#[derive(Serialize)]
pub struct Decklist(Class);
#[derive(Serialize)]
pub enum Class {
    Barbarian,
    Monk,
    Rogue,
    Mage,
}

impl Combat {
    pub fn dump(&self) {
        let buffer = File::create("./combat-state.yaml").unwrap();
        serde_yaml::to_writer(buffer, &self).unwrap();
    }
}

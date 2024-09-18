use std::collections::BTreeMap;

use rusqlite::Connection;
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub resources: BTreeMap<Resource, i64>,
    pub max_resources: BTreeMap<Resource, u64>,
    pub sleight_of_hand: BTreeMap<SleightOfHand, u16>,
    pub assisstances: BTreeMap<Assistance, u16>,
    pub resistances: BTreeMap<Resistance, u16>,
    pub buffs: BTreeMap<Buff, u16>,
    pub debuffs: BTreeMap<Debuff, u16>,
}

#[derive(Clone, Debug)]
#[orm_autobind]
pub struct StatChange {
    pub source: Player,
    pub target: Player,
    pub stat: Stat,
    pub change: i64,
}
impl StatChange {
    pub fn find_source(conn: &Connection, source: &Player) -> Vec<Self> {
        let (_id, stat_changes) = StatChangeDao::select_source(conn, source)
            .ok()
            .unwrap_or_else(|| panic!("Failed to find StatChanges for {:?}", source))
            .into_iter()
            .map(|stat| stat.into())
            .map(|(id, stat)| (id.expect("Selected StatChange with no Id"), stat))
            .unzip::<StatChangeId, StatChange, Vec<_>, Vec<_>>();
        stat_changes
    }

    pub fn find_target(conn: &Connection, target: &Player) -> Vec<Self> {
        let (_id, stat_changes) = StatChangeDao::select_target(conn, target)
            .ok()
            .unwrap_or_else(|| panic!("Failed to find StatChanges for {:?}", target))
            .into_iter()
            .map(|stat| stat.into())
            .map(|(id, stat)| (id.expect("Selected StatChange with no Id"), stat))
            .unzip::<StatChangeId, StatChange, Vec<_>, Vec<_>>();
        stat_changes
    }
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

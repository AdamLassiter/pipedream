use std::collections::BTreeMap;

use rusqlite::Connection;
use rusqlite_orm::orm_autobind;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::player::Player;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub resources: BTreeMap<Resource, i64>,
    pub max_resources: BTreeMap<Resource, u64>,
    pub sleight_of_hand: BTreeMap<SleightOfHand, u16>,
    pub assistances: BTreeMap<Assistance, u16>,
    pub resistances: BTreeMap<Resistance, u16>,
    pub buffs: BTreeMap<Buff, u16>,
    pub debuffs: BTreeMap<Debuff, u16>,
}
impl Default for Stats {
    fn default() -> Self {
        Self {
            resources: Resource::iter()
                .map(|r| (r, 10))
                .collect::<BTreeMap<_, _>>(),
            max_resources: Resource::iter()
                .map(|r| (r, 10))
                .collect::<BTreeMap<_, _>>(),
            sleight_of_hand: SleightOfHand::iter()
                .map(|s| (s, 10))
                .collect::<BTreeMap<_, _>>(),
            assistances: Assistance::iter()
                .map(|a| (a, 10))
                .collect::<BTreeMap<_, _>>(),
            resistances: Resistance::iter()
                .map(|r| (r, 10))
                .collect::<BTreeMap<_, _>>(),
            buffs: Buff::iter().map(|b| (b, 0)).collect::<BTreeMap<_, _>>(),
            debuffs: Debuff::iter().map(|d| (d, 0)).collect::<BTreeMap<_, _>>(),
        }
    }
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
            .unwrap_or_else(|e| panic!("Failed to find StatChanges for {:?}: {}", source, e))
            .into_iter()
            .map(|stat| stat.into())
            .map(|(id, stat)| (id.expect("Selected StatChange with no Id"), stat))
            .unzip::<StatChangeId, StatChange, Vec<_>, Vec<_>>();
        stat_changes
    }

    pub fn find_target(conn: &Connection, target: &Player) -> Vec<Self> {
        let (_id, stat_changes) = StatChangeDao::select_target(conn, target)
            .unwrap_or_else(|e| panic!("Failed to find StatChanges for {:?}: {}", target, e))
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

#[derive(
    Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter,
)]
pub enum Resource {
    Health,
    Stamina,
    Mana,
    Favour,
}
impl Resource {
    pub fn style(&self) -> &'static str {
        match self {
            Resource::Health => "red",
            Resource::Stamina => "green",
            Resource::Mana => "blue",
            Resource::Favour => "yellow",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum SleightOfHand {
    Inspiration,  // Draw count
    Versatility,  // Hand size
    Tranquility,  // Deck min
    Recollection, // Deck max
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Assistance {
    Strength,     // Physical
    Dexterity,    // Poisons
    Intelligence, // Magic
    Faith,        // Divine
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Resistance {
    Endurance,
    Vitality,
    Adaptibility,
    Fortitude,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    #[serde(default = "BTreeMap::new")]
    pub resources: BTreeMap<Resource, i64>,
    #[serde(default = "BTreeMap::new")]
    pub max_resources: BTreeMap<Resource, u64>,
    #[serde(default = "BTreeMap::new")]
    pub sleight_of_hand: BTreeMap<SleightOfHand, u16>,
    #[serde(default = "BTreeMap::new")]
    pub assistances: BTreeMap<Assistance, u16>,
    #[serde(default = "BTreeMap::new")]
    pub resistances: BTreeMap<Resistance, u16>,
    #[serde(default = "BTreeMap::new")]
    pub buffs: BTreeMap<Buff, u16>,
    #[serde(default = "BTreeMap::new")]
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

#[derive(Serialize, Deserialize)]
pub enum Stat {
    SleightOfHand(SleightOfHand),
    Element(Element),
    Assistance(Assistance),
    Resistance(Resistance),
    Buff(Buff),
    Debuff(Debuff),
}

#[derive(EnumIter, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Resource {
    Health,
    Stamina,
    Mana,
    Favour,
}

#[derive(EnumIter, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum SleightOfHand {
    Inspiration,  // Draw count
    Versatility,  // Hand size
    Tranquility,  // Deck min
    Recollection, // Deck max
}

#[derive(EnumIter, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Assistance {
    Strength,
}

#[derive(EnumIter, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Resistance {
    Endurance,
}

#[derive(EnumIter, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Element {
    Bludgeoning,
}

#[derive(EnumIter, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Buff {
    Overwhelm,
}

#[derive(EnumIter, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Debuff {
    Stun,
}

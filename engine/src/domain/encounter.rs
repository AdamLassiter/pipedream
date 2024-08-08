use rusqlite_orm::orm_bind;
use serde::{Deserialize, Serialize};

use super::{card::Card, character::Character, field::FieldPlace};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player {
    Human,
    Cpu,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[orm_bind {player: "$.player", place: "$.place", name: "$.card.choice.summary"}]
pub struct EncounterCard {
    player: Player,
    place: FieldPlace,
    card: Card,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[orm_bind {player: "$.player", name: "$.character.name"}]
pub struct EncounterCharacter {
    player: Player,
    character: Character,
}

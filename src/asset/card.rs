use std::collections::BTreeMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::asset::stats::Stats;

#[derive(Resource)]
pub struct CardLibraryHandle(pub Handle<CardLibrary>);

#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct CardLibrary(pub BTreeMap<CardId, Card>);

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CardId(pub String);

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub card_type: CardType,
    pub cost: Stats,
    pub deal: Stats,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub enum CardType {
    Combat,
    Negotiation,
    Hybrid,
}

pub enum CardPlace {
    Innate,
    Deck,
    Hand,
    Discard,
    Vanish,
}

pub struct Deck {
    pub deck: BTreeMap<CardPlace, Vec<CardId>>,
}

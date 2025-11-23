use std::collections::BTreeMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::asset::stats::Stats;

#[derive(Resource)]
pub struct ActorLibraryHandle(pub Handle<ActorLibrary>);

#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct ActorLibrary(pub BTreeMap<ActorId, Actor>);

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActorId(pub String);

#[derive(Serialize, Deserialize)]
pub struct Actor {
    pub name: String,
    pub actor_type: ActorType,
    pub stats: Stats,
    pub image: String,
    pub face_image: String,
}

#[derive(Serialize, Deserialize)]
pub enum ActorType {
    Player,
    Enemy,
    Fate,
}

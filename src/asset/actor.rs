use bevy::prelude::*;

pub struct ActorId(pub String);

pub enum ActorType {
    Player,
    Enemy,
}

pub struct Actor {
    pub id: ActorId,
    pub actor_type: ActorType,
    pub name: String,
    pub image: String,
}

use bevy::prelude::*;

#[derive(Component)]
pub enum CardPlace {
    Innate,
    Deck,
    Hand,
    Discard,
    Vanish,
}

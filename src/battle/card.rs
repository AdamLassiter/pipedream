use bevy::prelude::*;

#[derive(Component)]
pub struct Card {
    pub title: String,
    pub cost: String,
    pub details: String,
    pub image: String,
    pub selectable: bool,
}

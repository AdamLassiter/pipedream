use bevy::prelude::*;

#[derive(Component)]
pub struct Location {
    pub title: String,
    pub details: Vec<String>,
    pub image: Image,
    pub selectable: bool,
}

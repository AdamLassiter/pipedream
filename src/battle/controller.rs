use bevy::prelude::*;

#[derive(Component)]
pub enum Controller {
    Human,
    Cpu,
    World,
}

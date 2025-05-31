use bevy::prelude::*;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MenuState {
    #[default]
    None,
    MainMenu,
    Settings,
}

use bevy::prelude::*;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    Menu,
    Campaign,
    Battle,
    GameOver,
}

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MenuState {
    None,
    #[default]
    MainMenu,
    Settings,
}

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum BattleState {
    #[default]
    Begin,
    Human,
    Cpu,
    World,
    End,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<MenuState>()
            .init_state::<BattleState>();
    }
}

use bevy::prelude::*;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

use crate::state::AppState;

fn setup_battle_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Script::<LuaScript>::new(
        asset_server.load("scripts/spawn.lua"),
    ));
}

fn handle_battle_ui(mut commands: Commands) {}

fn teardown_battle_ui(mut commands: Commands) {}

#[derive(Default)]
pub struct BattleUiPlugin;

impl Plugin for BattleUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Battle), setup_battle_ui)
            .add_systems(Update, handle_battle_ui)
            .add_systems(OnExit(AppState::Battle), teardown_battle_ui);
    }
}

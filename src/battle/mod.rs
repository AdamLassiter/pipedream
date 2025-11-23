use bevy::prelude::*;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

use crate::cards::DropZoneNode;
use crate::state::AppState;
use crate::{HEIGHT, WIDTH};

fn setup_battle_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Script::<LuaScript>::new(
        asset_server.load("scripts/spawn_battle.lua"),
    ));
    commands.spawn(DropZoneNode::new(
        vec2(0., -HEIGHT / 2. + 32.),
        vec2(WIDTH - 32., 50.),
    ));
}

fn handle_battle_ui(mut commands: Commands) {}

fn teardown_battle_ui(mut commands: Commands) {}

#[derive(Default)]
pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Battle), setup_battle_ui)
            .add_systems(Update, handle_battle_ui)
            .add_systems(OnExit(AppState::Battle), teardown_battle_ui);
    }
}

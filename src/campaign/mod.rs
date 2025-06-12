pub mod location;

use bevy::prelude::*;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

use crate::state::AppState;

fn setup_campaign_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Script::<LuaScript>::new(
        asset_server.load("scripts/spawn.lua"),
    ));
}

fn handle_campaign_ui(mut commands: Commands) {}

fn teardown_campaign_ui(mut commands: Commands) {}

#[derive(Default)]
pub struct CampaignUiPlugin;

impl Plugin for CampaignUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Campaign), setup_campaign_ui)
            .add_systems(Update, handle_campaign_ui)
            .add_systems(OnExit(AppState::Campaign), teardown_campaign_ui);
    }
}
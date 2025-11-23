pub mod location;

use bevy::prelude::*;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

use crate::state::AppState;

fn setup_campaign(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Script::<LuaScript>::new(
        asset_server.load("scripts/spawn_campaign.lua"),
    ));
}

fn handle_campaign(mut commands: Commands) {}

fn teardown_campaign(mut commands: Commands) {}

#[derive(Default)]
pub struct CampaignPlugin;

impl Plugin for CampaignPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Campaign), setup_campaign)
            .add_systems(Update, handle_campaign)
            .add_systems(OnExit(AppState::Campaign), teardown_campaign);
    }
}

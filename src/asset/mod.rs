use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::asset::{
    actor::{ActorLibrary, ActorLibraryHandle},
    card::{CardLibrary, CardLibraryHandle},
};

pub mod actor;
pub mod card;
pub mod class;
pub mod lore;
pub mod stats;

pub enum AssetData {
    Actors,
    Cards,
}
impl AssetData {
    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Actors => "data/actors.json",
            Self::Cards => "data/cards.json",
        }
    }
}

fn load_asset_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let actors = ActorLibraryHandle(asset_server.load(AssetData::Actors.asset_path()));
    commands.insert_resource(actors);

    let cards = CardLibraryHandle(asset_server.load(AssetData::Cards.asset_path()));
    commands.insert_resource(cards);
}

#[derive(Default)]
pub struct AssetDataPlugin;

impl Plugin for AssetDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<ActorLibrary>::new(&[
            AssetData::Actors.asset_path()
        ]))
        .add_plugins(JsonAssetPlugin::<CardLibrary>::new(&[
            AssetData::Cards.asset_path()
        ]))
        .add_systems(PostStartup, load_asset_data);
    }
}

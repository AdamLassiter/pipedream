use bevy::prelude::*;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

use crate::asset::card::{CardId, CardLibrary, CardLibraryHandle};
use crate::cards::{CardBacking, InteractiveNode};
use crate::zindex::{Z_CARD, Z_D_CARD_ICON};

#[derive(Component)]
struct SpawnedByScript;

#[derive(Component)]
struct BackgroundNode;

fn spawn_card(
    In((id,)): In<(String,)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cards_handle: Res<CardLibraryHandle>,
    cards: Res<Assets<CardLibrary>>,
) {
    if let Some(cards) = cards.get(cards_handle.0.id())
        && let Some(card) = cards.0.get(&CardId(id.clone()))
    {
        let backing = CardBacking::default();
        let backing_image = asset_server.load(backing.image.clone());
        let card_image = asset_server.load(card.image.clone());
        commands
            .spawn((
                InteractiveNode::default(),
                SpawnedByScript,
                Sprite::from_image(backing_image),
                Transform::from_translation(vec3(0., 0., Z_CARD)),
            ))
            .with_child((
                backing,
                SpawnedByScript,
                Sprite::from_image(card_image),
                Transform::from_translation(vec3(0., 0., Z_CARD + Z_D_CARD_ICON)),
            ));
    } else {
        log::warn!("Missing asset while spawning {}", id.clone());
    }
}

fn spawn_background(
    In((path,)): In<(String,)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load(path);
    commands.spawn((
        BackgroundNode,
        SpawnedByScript,
        Sprite { image, ..default() },
        Transform {
            translation: vec3(0., 0., 1.),
            ..default()
        },
    ));
}

fn teardown_all(
    mut commands: Commands,
    scripted_spawns_query: Query<Entity, With<SpawnedByScript>>,
) {
    for menu_entity in &scripted_spawns_query {
        commands.entity(menu_entity).despawn();
    }
}

pub struct ScriptsPlugin;

impl Plugin for ScriptsPlugin {
    fn build(&self, app: &mut App) {
        app.add_scripting_api::<LuaRuntime>(|rt| {
            rt.add_function("spawn_card".into(), spawn_card)
                .add_function("spawn_background".into(), spawn_background)
                .add_function("despawn_all".into(), teardown_all);
        });
    }
}

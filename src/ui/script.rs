use bevy::prelude::*;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

use super::interactive_sprites::InteractiveNode;

#[derive(Component)]
struct SpawnedByScript;

#[derive(Component)]
struct BackgroundNode;

fn spawn_interactive(
    In((path,)): In<(String,)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load(path);
    commands.spawn((
        InteractiveNode,
        SpawnedByScript,
        Sprite::from_image(image.clone()),
        Transform {
            translation: vec3(0., 0., 3.),
            ..default()
        },
    ));
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
            rt.add_function("spawn_interactive".into(), spawn_interactive)
                .add_function("spawn_background".into(), spawn_background)
                .add_function("despawn_all".into(), teardown_all);
        });
    }
}

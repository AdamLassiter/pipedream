use bevy::prelude::*;

use crate::ui::{resource::Language, state::MenuState};

use super::{BACKDROP, MenuButtonAction, NORMAL_BUTTON, TEXT};

#[derive(Component)]
pub struct OnSettingsMenuScreen;

fn setup_settings_menu(
    mut commands: Commands,
    language: Res<Language>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(language.font_path());
    let background = asset_server.load("backgrounds/mountain/Mountains4/Bright/mountains4.png");

    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        font: font.clone(),
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSettingsMenuScreen,
        super::OnMenuScreen,
        ImageNode::new(background),
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Text::new(" settings "),
                    TextFont {
                        font_size: 67.0,
                        font,
                        ..default()
                    },
                    TextColor(TEXT),
                    BackgroundColor(BACKDROP),
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::MainMenu,
                    children![(Text::new("back"), button_text_font, TextColor(TEXT),),]
                ),
            ]
        )],
    ));
}

pub fn teardown_settings_menu(
    mut commands: Commands,
    settings_items_query: Query<Entity, With<OnSettingsMenuScreen>>,
) {
    for menu_entity in &settings_items_query {
        commands.entity(menu_entity).despawn();
    }
}

#[derive(Default)]
pub struct SettingsMenuUiPlugin;

impl Plugin for SettingsMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Settings), setup_settings_menu)
            .add_systems(OnExit(MenuState::Settings), teardown_settings_menu);
    }
}

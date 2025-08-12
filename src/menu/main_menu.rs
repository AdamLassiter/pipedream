use bevy::prelude::*;

use crate::{state::MenuState, ui::resource::Language};

use super::{BACKDROP, MenuButtonAction, NORMAL_BUTTON, TEXT};

#[derive(Component)]
pub struct OnMainMenuScreen;

fn setup_main_menu(
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
        OnMainMenuScreen,
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
                    Text::new(" pipedream "),
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
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![(
                        Text::new("new game"),
                        button_text_font.clone(),
                        TextColor(TEXT),
                    ),]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Settings,
                    children![(
                        Text::new("settings"),
                        button_text_font.clone(),
                        TextColor(TEXT),
                    ),]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![(Text::new("quit"), button_text_font, TextColor(TEXT),),]
                ),
            ]
        )],
    ));
}

pub fn teardown_main_menu(
    mut commands: Commands,
    menu_items_query: Query<Entity, With<OnMainMenuScreen>>,
) {
    for menu_entity in &menu_items_query {
        commands.entity(menu_entity).despawn();
    }
}

#[derive(Default)]
pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::MainMenu), setup_main_menu)
            .add_systems(OnExit(MenuState::MainMenu), teardown_main_menu);
    }
}

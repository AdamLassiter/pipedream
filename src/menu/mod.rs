use bevy::prelude::*;

use crate::{
    menu::resource::ResourcesPlugin,
    state::{AppState, MenuState},
};

mod main_menu;
mod resource;
mod settings;

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    MainMenu,
    Settings,
    Quit,
}
#[derive(Component)]
struct SelectedButton;

const TEXT: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::srgba(0.15, 0.15, 0.15, 0.9);
const HOVERED_BUTTON: Color = Color::srgba(0.25, 0.25, 0.25, 0.9);
const HOVERED_PRESSED_BUTTON: Color = Color::srgba(0.25, 0.65, 0.25, 0.9);
const PRESSED_BUTTON: Color = Color::srgba(0.35, 0.75, 0.35, 0.9);
const BACKDROP: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);

fn handle_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedButton>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn handle_menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    app_state.set(AppState::Battle);
                }
                MenuButtonAction::MainMenu => {
                    menu_state.set(MenuState::MainMenu);
                }
                MenuButtonAction::Settings => {
                    menu_state.set(MenuState::Settings);
                }
                MenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
            }
        }
    }
}

fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::MainMenu);
}

fn teardown_menu(
    mut commands: Commands,
    menu_items_query: Query<Entity, With<OnMenuScreen>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    menu_state.set(MenuState::None);
    for menu_entity in &menu_items_query {
        commands.entity(menu_entity).despawn();
    }
}

#[derive(Default)]
pub struct MenuUiPlugin;

impl Plugin for MenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_plugins(ResourcesPlugin)
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_plugins((main_menu::MainMenuUiPlugin, settings::SettingsMenuUiPlugin))
            .add_systems(PreUpdate, (handle_button_interaction, handle_menu_action))
            .add_systems(OnExit(AppState::Menu), teardown_menu);
    }
}

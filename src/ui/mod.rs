use bevy::prelude::*;

pub mod event;
pub mod resource;
pub mod script;
pub mod state;

mod interactive_sprites;

#[derive(Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            event::EventsPlugin,
            script::ScriptsPlugin,
            resource::ResourcesPlugin,
            interactive_sprites::InteractiveSpritesPlugin,
        ));
    }
}

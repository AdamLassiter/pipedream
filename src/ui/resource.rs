use bevy::prelude::*;

#[derive(Resource, Component, Default)]
pub enum Language {
    #[default]
    Futhark,
}
impl Language {
    pub fn font_path(&self) -> &'static str {
        match self {
            Language::Futhark => "fonts/ElderFuthark.ttf",
        }
    }
}

fn load_fonts(language: Res<Language>, asset_server: Res<AssetServer>) {
    let _font: Handle<Font> = asset_server.load(language.font_path());
}

#[derive(Default)]
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Language>()
            .add_systems(Startup, load_fonts);
    }
}

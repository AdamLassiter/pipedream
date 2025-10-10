use bevy::prelude::*;
use bevy_pixcam::PixelCameraPlugin;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

use pipedream::{
    campaign::CampaignUiPlugin, event::EventsPlugin, menu::MenuUiPlugin, setup_app,
    state::StatesPlugin, ui::UiPlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        PixelCameraPlugin,
    ))
    .add_scripting::<LuaRuntime>(|_| {
        // instantiated through ::add_scripting_api
    })
    .add_plugins((
        UiPlugin,
        EventsPlugin,
        StatesPlugin,
        MenuUiPlugin,
        CampaignUiPlugin,
    ))
    .add_systems(Startup, setup_app);

    #[cfg(feature = "dev_mode")]
    {
        app.add_plugins(bevy::dev_tools::fps_overlay::FpsOverlayPlugin {
            ..Default::default()
        });
    }

    app.run();
}

#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use bevy_pixcam::{PixelViewport, PixelZoom};

pub mod battle;
pub mod campaign;
pub mod event;
pub mod state;
pub mod ui;

extern crate log;


#[derive(Component)]
pub struct MainCamera;

pub fn setup_app(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera,
        PixelViewport,
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
    ));
}

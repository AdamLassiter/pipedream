#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod asset;
pub mod battle;
pub mod campaign;
pub mod cards;
pub mod menu;
pub mod state;
pub mod zindex;

use bevy::prelude::*;
use bevy_pixcam::{PixelViewport, PixelZoom};

extern crate log;

const WIDTH: f32 = 576.;
const HEIGHT: f32 = 324.;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_app(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera,
        PixelViewport,
        PixelZoom::FitSize {
            width: WIDTH as i32,
            height: HEIGHT as i32,
        },
    ));
}

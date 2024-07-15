use crate::{Handler, Renderable};

pub mod campaign;
pub mod inventory;
pub mod logging;
mod scene_and_choices;
pub mod combat;

pub trait Component: Handler + Renderable + Send {}

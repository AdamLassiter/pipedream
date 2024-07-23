use crate::{Handler, Renderable};

mod campaign;
mod combat;
pub mod inventory;
pub mod logging;
pub mod scene_and_choices;

pub trait Component: Handler + Renderable + Send {}

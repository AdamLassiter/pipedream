use crate::{Handler, Renderable};

pub mod inventory;
pub mod logging;
pub mod messages;
pub mod scene;

pub trait Component: Handler + Renderable + Send {}

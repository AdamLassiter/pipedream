use crate::{Handler, Renderable};

pub mod campaign;
pub mod inventory;
pub mod logging;

pub trait Component: Handler + Renderable + Send {}

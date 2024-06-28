use handler::Handler;

pub mod tui;
pub mod handler;
pub mod utils;
pub mod widget;

trait Component: Handler + Send {}

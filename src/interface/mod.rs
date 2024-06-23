use handler::Handler;

pub mod tui;
pub mod handler;
pub mod utils;
pub mod widgets;

trait Component: Handler + Send {}

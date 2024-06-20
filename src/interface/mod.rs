use handler::Handler;

pub mod app;
pub mod handler;
pub mod utils;
pub mod widgets;

trait Component: Handler + Send {}

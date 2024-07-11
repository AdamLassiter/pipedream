#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

use crossterm::event::KeyEvent;
use handler::Handler;
use pipedream_engine::core::{choice::Choice, transition::Transition};
use ratatui::{buffer::Buffer, layout::Rect};

pub mod handler;
pub mod tui;
pub mod utils;
pub mod widget;

// Extern log for arbitrary provider
extern crate log;

trait Component: Handler + Send {}

pub trait Controllable {
    fn handle_key_event(&mut self, key_event: KeyEvent);

    fn current_choice(&self) -> Option<Choice>;

    fn current_transition(&self) -> Option<Transition>;
}

pub trait Renderable {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

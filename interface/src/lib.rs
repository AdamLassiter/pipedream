#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

use std::time::Duration;

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use pipedream_domain::{choice::Choice, effect::Effect};

pub mod ascii_art;
pub mod component;
pub mod log_utils;
pub mod tui;
pub mod widget;

extern crate log;

const CULL_POLL_INTERVAL: Duration = Duration::from_millis(10);
const EVENT_POLL_INTERVAL: Duration = Duration::from_millis(10);

pub trait Controllable {
    fn handle_key_event(&mut self, key_event: KeyEvent);

    fn current_choice(&self) -> Option<Choice>;

    fn current_transition(&self) -> Option<Effect>;
}

pub trait Renderable {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

pub trait Handler {
    fn handle_key_event(&mut self, key_event: KeyEvent);

    fn handle_tick_event(&mut self) -> TickResult;
}

pub struct TickResult {
    pub should_redraw: bool,
}

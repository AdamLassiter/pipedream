#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

use crossterm::event::KeyEvent;
use pipedream_engine::{core::{choice::Choice, commands::{EngineCommand, UiCommand}, transition::Transition}, bichannel::Channel};
use ratatui::{buffer::Buffer, layout::Rect};

pub mod component;
pub mod image;
pub mod tui;
pub mod log_utils;
pub mod widget;

pub trait Controllable {
    fn handle_key_event(&mut self, key_event: KeyEvent);

    fn current_choice(&self) -> Option<Choice>;

    fn current_transition(&self) -> Option<Transition>;
}

pub trait Renderable {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

pub trait Handler {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        channel: &Channel<EngineCommand, UiCommand>,
    );

    fn handle_tick_event(&mut self, channel: &Channel<EngineCommand, UiCommand>) -> bool;
}

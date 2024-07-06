pub mod logging_handler;
pub mod campaign_handler;

use bichannel::Channel;
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::engine::core::commands::{EngineCommand, UiCommand};

pub trait Handler {
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        channel: &Channel<EngineCommand, UiCommand>,
    );

    fn handle_tick_event(&mut self, channel: &Channel<EngineCommand, UiCommand>) -> bool;

    fn handle_render(&self, area: Rect, buf: &mut Buffer);
}

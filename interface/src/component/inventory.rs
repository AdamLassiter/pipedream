use crate::{Handler, Renderable, TickResult};
use crossterm::event::KeyEvent;
use pipedream_bichannel::Bichannel;
use pipedream_engine::core::{
    command::{EngineCommand, UiCommand},
    tags::Tags,
};
use ratatui::prelude::*;

use super::Component;

pub struct InventoryComponent {
    channel: Bichannel<EngineCommand, UiCommand>,
    tags: Option<Tags>,
}

impl InventoryComponent {
    pub fn new(channel: Bichannel<EngineCommand, UiCommand>) -> Self {
        Self {
            channel,
            tags: None,
        }
    }
}

impl Handler for InventoryComponent {
    fn handle_tick_event(&mut self) -> TickResult {
        let mut should_redraw = false;

        while let Ok(ev) = self.channel.try_recv() {
            match ev {
                UiCommand::ShowTags(tags) => self.tags = Some(tags),
                _ => {/* Do nothing */}
            }
            should_redraw = true;
        }

        TickResult {
            should_redraw,
        }
    }

    fn handle_key_event(&mut self, _key_event: KeyEvent) {
        // None
    }
}

impl Renderable for InventoryComponent {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if let Some(tags) = self.tags.as_ref() {
            tags.render(area, buf);
        }
    }
}

impl Component for InventoryComponent {}

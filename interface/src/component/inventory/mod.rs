mod campaign;
mod combat;

use crate::{Handler, Renderable, TickResult};
use crossterm::event::KeyEvent;
use pipedream_bichannel::Bichannel;
use pipedream_engine::core::{
    command::{EngineCommand, UiCommand, UiMode},
    tags::Tags,
};
use ratatui::prelude::*;

use super::Component;

pub struct InventoryComponent {
    channel: Bichannel<EngineCommand, UiCommand>,
    player_tags: Option<Tags>,
    enemy_tags: Option<Tags>,
    ui_mode: UiMode,
}

impl InventoryComponent {
    pub fn new(channel: Bichannel<EngineCommand, UiCommand>) -> Self {
        Self {
            channel,
            player_tags: None,
            enemy_tags: None,
            ui_mode: UiMode::Campaign,
        }
    }
}

impl Handler for InventoryComponent {
    fn handle_tick_event(&mut self) -> TickResult {
        let mut should_redraw = false;

        while let Ok(ev) = self.channel.try_recv() {
            match ev {
                UiCommand::ShowTags(tags) => self.player_tags = Some(tags),
                UiCommand::ChangeMode(mode) => {
                    self.ui_mode = mode;
                }
                _ => { /* Do nothing */ }
            }
            should_redraw = true;
        }

        TickResult { should_redraw }
    }

    fn handle_key_event(&mut self, _key_event: KeyEvent) {
        // None
    }
}

impl Renderable for InventoryComponent {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        match self.ui_mode {
            UiMode::Campaign => self.render_campaign(area, buf),
            UiMode::Combat => self.render_combat(area, buf),
        }
    }
}

impl Component for InventoryComponent {}

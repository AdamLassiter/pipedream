mod campaign;
mod combat;

use crate::{Handler, Renderable, TickResult};
use bichannel::Bichannel;
use crossterm::event::KeyEvent;
use pipedream_domain::stats::Stats;
use pipedream_engine::{
    command::{EngineCommand, UiCommand, UiMode},
    image::Image,
};
use ratatui::prelude::*;

use super::Component;

pub struct InventoryComponent {
    channel: Bichannel<EngineCommand, UiCommand>,
    player_image: Option<Image>,
    enemy_image: Option<Image>,
    player_stats: Option<Stats>,
    enemy_stats: Option<Stats>,
    ui_mode: UiMode,
}

impl InventoryComponent {
    pub fn new(channel: Bichannel<EngineCommand, UiCommand>) -> Self {
        Self {
            channel,
            player_image: None,
            enemy_image: None,
            player_stats: None,
            enemy_stats: None,
            ui_mode: UiMode::Campaign,
        }
    }
}

impl Handler for InventoryComponent {
    fn handle_tick_event(&mut self) -> TickResult {
        let mut should_redraw = false;

        while let Ok(ev) = self.channel.try_recv() {
            match ev {
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

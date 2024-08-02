mod campaign;
mod combat;

use crate::{widget::tags::IMAGE_TAGS, Handler, Renderable, TickResult};
use crossterm::event::KeyEvent;
use pipedream_bichannel::Bichannel;
use pipedream_engine::{
    combat::target::Tgt,
    core::{
        command::{EngineCommand, UiCommand, UiMode},
        image::Image,
        tags::Tags,
    },
};
use ratatui::prelude::*;

use super::Component;

pub struct InventoryComponent {
    channel: Bichannel<EngineCommand, UiCommand>,
    tags: Option<Tags>,
    player_image: Option<Image>,
    enemy_image: Option<Image>,
    ui_mode: UiMode,
}

impl InventoryComponent {
    pub fn new(channel: Bichannel<EngineCommand, UiCommand>) -> Self {
        Self {
            channel,
            tags: None,
            player_image: None,
            enemy_image: None,
            ui_mode: UiMode::Campaign,
        }
    }
}

impl Handler for InventoryComponent {
    fn handle_tick_event(&mut self) -> TickResult {
        let mut should_redraw = false;

        while let Ok(ev) = self.channel.try_recv() {
            match ev {
                UiCommand::ShowTags(tags) => {
                    // Player
                    if let Some(portrait_tag) = tags
                        .find(
                            IMAGE_TAGS
                                .get(&Tgt::Player)
                                .expect("Failed to get tag-key for player portrait"),
                        )
                        .first()
                    {
                        self.player_image =
                            Some(Image(portrait_tag.key.trailing_key().to_string()));
                    }
                    // Enemy
                    if let Some(portrait_tag) = tags
                        .find(
                            IMAGE_TAGS
                                .get(&Tgt::Enemy)
                                .expect("Failed to get tag-key for enemy portrait"),
                        )
                        .first()
                    {
                        self.enemy_image = Some(Image(portrait_tag.key.trailing_key().to_string()));
                    }
                    self.tags = Some(tags);
                }
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

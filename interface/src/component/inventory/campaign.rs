use pipedream_engine::combat::{entity::Ent, target::Tgt};
use ratatui::prelude::*;

use crate::{widget::tags::TgtEntTags, Renderable};

use super::InventoryComponent;

impl InventoryComponent {
    pub fn render_campaign(&self, area: Rect, buf: &mut Buffer) {
        if let Some(tags) = self.player_stats.as_ref() {
            TgtEntTags {
                tgt: Tgt::Player,
                ent: Ent::Resource,
                tags,
            }
            .render(area, buf);
        }
    }
}

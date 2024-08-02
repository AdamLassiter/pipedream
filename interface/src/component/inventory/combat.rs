use pipedream_engine::combat::{entity::Ent, target::Tgt};
use ratatui::prelude::*;

use crate::{widget::tags::TgtEntTags, Renderable};

use super::InventoryComponent;

impl InventoryComponent {
    pub fn render_combat(&self, area: Rect, buf: &mut Buffer) {
        let [player_area, enemy_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        if let Some(tags) = self.tags.as_ref() {
            TgtEntTags {
                tgt: Tgt::Player,
                ent: Ent::Resource,
                tags,
            }
            .render(player_area, buf);
        }

        if let Some(tags) = self.tags.as_ref() {
            TgtEntTags {
                tgt: Tgt::Enemy,
                ent: Ent::Resource,
                tags,
            }
            .render(enemy_area, buf);
        }
    }
}

use pipedream_engine::game::{entity::Ent, target::Tgt};
use ratatui::prelude::*;

use crate::{widget::tags::TgtEntTags, Renderable};

use super::InventoryComponent;

impl InventoryComponent {
    pub fn render_campaign(&self, area: Rect, buf: &mut Buffer) {
        if let Some(tags) = self.tags.as_ref() {
            let player_resources = TgtEntTags {
                tgt: Tgt::Player,
                ent: Ent::Resource,
                tags,
            };
            let player_items = TgtEntTags {
                tgt: Tgt::Player,
                ent: Ent::Item,
                tags,
            };

            // Layouts
            let [resources_area, items_area, _] = Layout::vertical([
                Constraint::Length(player_resources.size_hint()),
                Constraint::Length(player_items.size_hint()),
                Constraint::Fill(1),
            ])
            .areas(area);

            // Render
            player_resources.render(resources_area, buf);
            player_items.render(items_area, buf);
        }
    }
}

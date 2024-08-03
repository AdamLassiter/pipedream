use pipedream_engine::game::{entity::Ent, target::Tgt};
use ratatui::prelude::*;

use crate::{widget::tags::TgtEntTags, Renderable};

use super::InventoryComponent;

impl InventoryComponent {
    pub fn render_combat(&self, area: Rect, buf: &mut Buffer) {
        let [player_area, enemy_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

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
            let [player_resources_area, player_items_area, _] = Layout::vertical([
                Constraint::Length(player_resources.size_hint()),
                Constraint::Length(player_items.size_hint()),
                Constraint::Fill(1),
            ])
            .areas(player_area);

            // Render
            player_resources.render(player_resources_area, buf);
            player_items.render(player_items_area, buf);
        }

        if let Some(tags) = self.tags.as_ref() {
            let enemy_resources = TgtEntTags {
                tgt: Tgt::Enemy,
                ent: Ent::Resource,
                tags,
            };
            let enemy_items = TgtEntTags {
                tgt: Tgt::Player,
                ent: Ent::Item,
                tags,
            };

            // Layouts
            let [enemy_resources_area, enemy_items_area, _] = Layout::vertical([
                Constraint::Length(enemy_resources.size_hint()),
                Constraint::Length(enemy_items.size_hint()),
                Constraint::Fill(1),
            ])
            .areas(enemy_area);

            // Render
            enemy_resources.render(enemy_resources_area, buf);
            enemy_items.render(enemy_items_area, buf);
        }
    }
}

use crate::Renderable;
use ratatui::prelude::*;

use super::InventoryComponent;

impl InventoryComponent {
    pub fn render_campaign(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let [player_area, enemy_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        if let Some(tags) = self.player_tags.as_ref() {
            tags.render(player_area, buf);
        }

        if let Some(tags) = self.enemy_tags.as_ref() {
            tags.render(enemy_area, buf);
        }
    }
}

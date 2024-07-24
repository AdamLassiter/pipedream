use ratatui::prelude::*;

use crate::Renderable;

use super::InventoryComponent;

impl InventoryComponent {
    pub fn render_campaign(&self, area: Rect, buf: &mut Buffer) {
        let [player_area, enemy_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        if let Some(tags) = self.player_stats.as_ref() {
            (*tags).render(player_area, buf);
        }

        if let Some(tags) = self.enemy_stats.as_ref() {
            (*tags).render(enemy_area, buf);
        }
    }
}

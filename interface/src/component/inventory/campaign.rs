use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use symbols::border;

use crate::Renderable;

use super::InventoryComponent;

impl InventoryComponent {
    pub fn render_campaign(&self, area: Rect, buf: &mut Buffer) {
        // Size hints
        let portrait_width_hint = if self.player_image.is_some() {
            32 + 2
        } else {
            0
        };
        let portrait_height_hint = if self.player_image.is_some() {
            16 + 2
        } else {
            0
        };

        // Layouts
        let [portrait_and_stats_area, _inventory_area] = Layout::vertical([
            Constraint::Length(portrait_height_hint),
            Constraint::Fill(1),
        ])
        .areas(area);
        let [portrait_border_area, stats_border_area] =
            Layout::horizontal([Constraint::Length(portrait_width_hint), Constraint::Fill(1)])
                .areas(portrait_and_stats_area);

        // Render
        if let Some(portrait) = self.player_image.as_ref() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED);
            let portrait_area = block.inner(portrait_border_area);

            block.render(portrait_border_area, buf);
            portrait.render(portrait_area, buf);
        }
        if let Some(stats) = self.player_stats.as_ref() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED);
            let stats_area = block.inner(stats_border_area);

            block.render(stats_border_area, buf);
            stats.render(stats_area, buf);
        }
    }
}

use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use symbols::border;

use crate::Renderable;

use super::InventoryComponent;

impl InventoryComponent {
    pub fn render_combat(&self, area: Rect, buf: &mut Buffer) {
        // Size hints
        let (portrait_width_hint, portrait_height_hint) =
            if self.player_image.is_some() || self.enemy_image.is_some() {
                (32 + 2, 16 + 2)
            } else {
                (0, 0)
            };

        // Layouts
        let [stats_area, _inventory_area] = Layout::vertical([
            Constraint::Length(portrait_height_hint),
            Constraint::Fill(1),
        ])
        .areas(area);
        let [player_stats_area, enemy_stats_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(stats_area);
        let [player_portrait_border_area, player_stats_border_area] =
            Layout::horizontal([Constraint::Length(portrait_width_hint), Constraint::Fill(1)])
                .areas(player_stats_area);
        let [enemy_stats_border_area, enemy_portrait_border_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(portrait_width_hint)])
                .areas(enemy_stats_area);

        // Render
        if let Some(portrait) = self.player_image.as_ref() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED);
            let portrait_area = block.inner(player_portrait_border_area);

            block.render(player_portrait_border_area, buf);
            portrait.render(portrait_area, buf);
        }
        if let Some(stats) = self.player_stats.as_ref() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED);
            let stats_area = block.inner(player_stats_border_area);

            block.render(player_stats_border_area, buf);
            stats.render(stats_area, buf);
        }
        if let Some(portrait) = self.enemy_image.as_ref() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED);
            let portrait_area = block.inner(enemy_portrait_border_area);

            block.render(enemy_portrait_border_area, buf);
            portrait.render(portrait_area, buf);
        }
        if let Some(stats) = self.enemy_stats.as_ref() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED);
            let stats_area = block.inner(enemy_stats_border_area);

            block.render(enemy_stats_border_area, buf);
            stats.render(stats_area, buf);
        }
    }
}

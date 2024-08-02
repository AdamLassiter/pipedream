use pipedream_engine::combat::{entity::Ent, target::Tgt};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use symbols::border;

use crate::{widget::tags::TgtEntTags, Renderable};

use super::SceneComponent;

impl SceneComponent {
    pub fn render_combat(&self, area: Rect, buf: &mut Buffer) {
        // Size hints
        let portrait_width_hint = if self.image.is_some() { 32 + 2 } else { 0 };
        let portrait_height_hint = if self.image.is_some() { 16 + 2 } else { 0 };
        let scene_size_hint = self
            .scene
            .as_ref()
            .map(|s| s.descriptions.len())
            .unwrap_or(0) as u16;

        // Layouts
        let [stats_area, scene_area, cards_area] = Layout::vertical([
            Constraint::Length(portrait_height_hint),
            Constraint::Length(scene_size_hint),
            Constraint::Fill(1),
        ])
        .areas(area);
        let [player_stats_area, enemy_stats_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(stats_area);
        let [portrait_border_area, stats_area] =
            Layout::horizontal([Constraint::Length(portrait_width_hint), Constraint::Fill(1)])
                .areas(player_stats_area);

        // Render
        if let Some(scene) = self.scene.as_ref() {
            scene.render(scene_area, buf);
        }
        if let Some(portrait) = self.image.as_ref() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED);
            let portrait_area = block.inner(portrait_border_area);

            block.render(portrait_border_area, buf);
            portrait.render(portrait_area, buf);
        }
        if let Some(tags) = self.tags.as_ref() {
            TgtEntTags {
                tgt: Tgt::Player,
                ent: Ent::Resource,
                tags,
            }
            .render(stats_area, buf);
        }
        if let Some(choices) = self.options.as_ref() {
            choices.renderable().render(cards_area, buf);
        }
    }
}

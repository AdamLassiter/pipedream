use pipedream_engine::core::choice::Choices;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use symbols::border;

use crate::{widget::choice::campaign_choice::CampaignChoice, Renderable};

use super::SceneComponent;

impl SceneComponent {
    pub fn render_campaign(&self, area: Rect, buf: &mut Buffer) {
        // Size hints
        let scene_size_hint = self.scene.as_ref().map(|scene| scene.descriptions.len()).unwrap_or(0) as u16;
        let choices_size_hint = match self.choices.as_ref().map(|c| c.choices()) {
            Some(Choices::Manual(choices)) => choices.len(),
            _ => 0,
        } as u16;
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
        let [description_area, details_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);
        let [portrait_and_stats_area, scene_and_choices_area] = Layout::vertical([
            Constraint::Length(portrait_height_hint),
            Constraint::Fill(1),
        ])
        .areas(description_area);
        let [portrait_border_area, stats_area] =
            Layout::horizontal([Constraint::Length(portrait_width_hint), Constraint::Fill(1)])
                .areas(portrait_and_stats_area);
        let [scene_area, choices_area] = Layout::vertical([
            Constraint::Min(scene_size_hint),
            Constraint::Min(choices_size_hint),
        ])
        .areas(scene_and_choices_area);

        // Render
        if let Some(scene) = self.scene.as_ref() {
            scene.render(scene_area, buf);
        }
        if let Some(widget) = self.choices.as_ref() {
            widget.renderable().render(choices_area, buf);

            let cursor = widget.cursor();
            if let Choices::Manual(choices) = &widget.choices() {
                if let Some(selected) = choices.get(*cursor) {
                    CampaignChoice(selected.clone()).render(details_area, buf);
                }
            }
        }
        if let Some(portrait) = self.player_image.as_ref() {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED);
            let portrait_area = block.inner(portrait_border_area);

            block.render(portrait_border_area, buf);
            portrait.render(portrait_area, buf);
        }
    }
}

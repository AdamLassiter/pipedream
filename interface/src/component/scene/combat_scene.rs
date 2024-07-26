use pipedream_engine::core::choice::{ChoiceType, Choices};
use ratatui::prelude::*;

use super::SceneComponent;

impl SceneComponent {
    pub fn render_combat(&self, area: Rect, buf: &mut Buffer) {
        let stats_size_hint = 5_u16;
        let scene_size_hint = self
            .scene
            .as_ref()
            .map(|s| s.descriptions.len())
            .unwrap_or(0) as u16;
        let has_details = self
            .options
            .as_ref()
            .map(|c| c.choices())
            .map(|x| match x {
                Choices {
                    choices: ChoiceType::Manual(choices),
                    cursor,
                } => choices
                    .get(*cursor)
                    .map(|choice| choice.image.is_some())
                    .unwrap_or(false),
                _ => false,
            })
            .unwrap_or(false);

        let [stats_area, cards_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        let [player_stats_area, enemy_stats_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(stats_area);

        if let Some(choices) = self.options.as_ref() {
            choices.renderable().render(cards_area, buf);
        }
    }
}

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
        let choices_size_hint = match self.options.as_ref() {
            Some(Choices {
                choices: ChoiceType::Manual(c),
                ..
            }) => c.len(),
            _ => 0,
        } as u16;
        let has_details = self
            .options
            .as_ref()
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
        let details_size_hint = if has_details { 32 + 2 + 2 } else { 0 };

        let [stats_area, cards_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        let [player_stats_area, enemy_stats_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(stats_area);

        // let [description_area, details_area] =
        //     Layout::horizontal([Constraint::Fill(1), Constraint::Length(details_size_hint)])
        //         .areas(area);
        // let [stats_area, scene_area, choices_area] = Layout::vertical([
        //     Constraint::Length(stats_size_hint),
        //     Constraint::Min(scene_size_hint),
        //     Constraint::Min(choices_size_hint),
        // ])
        // .areas(description_area);

        // if let Some(scene) = self.scene.as_ref() {
        //     scene.render(scene_area, buf);
        // }
        // if let Some(tags) = self.tags.as_ref() {
        //     tags.render(stats_area, buf);
        // }
        // if let Some(Choices {
        //     choices: ChoiceType::Manual(choices),
        //     cursor,
        // }) = self.options.as_ref()
        // {
        //     (choices.as_slice(), *cursor).render(choices_area, buf);
        //     if let Some(selected) = choices.get(*cursor) {
        //         selected.render(details_area, buf);
        //     }
        // }
    }
}

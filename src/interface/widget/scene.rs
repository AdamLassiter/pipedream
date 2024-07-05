use ratatui::{prelude::*, widgets::*};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use crate::engine::core::{description::Description, scene::Scene};

impl Widget for &Scene {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut scene = Text::default();
        self.descriptions
            .iter()
            .map(|Description { descriptor, .. }| {
                compile::<RatatuiTextGenerator>(descriptor)
                    .expect("Failed to compile tui text markup")
            })
            .for_each(|scene_line| scene.extend(scene_line));

        Widget::render(Paragraph::new(scene), area, buf);
    }
}

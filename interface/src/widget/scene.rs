use ratatui::{prelude::*, widgets::*};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use pipedream_engine::core::{description::Description, scene::Scene};

use crate::Renderable;

impl Renderable for Scene {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // debug!(target:"Render/Scene", "{:?}", self.descriptions);

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

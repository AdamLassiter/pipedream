use ratatui::{prelude::*, widgets::*};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use pipedream_engine::{
    core::{description::Description, scene::Scene},
    log::debug,
};

use crate::Renderable;

impl Renderable for Scene {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        debug!(target:"Interface/Scene/Render", "{:?}", self.0);

        let scene = self
            .0
            .iter()
            .map(|Description { descriptor, .. }| {
                compile::<RatatuiTextGenerator>(descriptor)
                    .expect("Failed to compile tui text markup")
            })
            .flat_map(|scene_text| scene_text.lines)
            .collect::<Vec<_>>();

        Widget::render(Paragraph::new(scene), area, buf);
    }
}

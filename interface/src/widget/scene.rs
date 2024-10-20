use log::debug;
use ratatui::{prelude::*, widgets::*};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use crate::Renderable;
use pipedream_domain::description::Description;
use pipedream_engine::scene::Scene;

impl Renderable for Scene {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        debug!(target:"Interface/Scene/Render", "{:?}", self);

        let scene = self
            .descriptions
            .iter()
            .map(|Description { descriptor, .. }| {
                compile::<RatatuiTextGenerator>(descriptor)
                    .expect("Failed to compile Tui text markup")
            })
            .flat_map(|scene_text| scene_text.lines)
            .collect::<Vec<_>>();

        Widget::render(Paragraph::new(scene), area, buf);
    }
}

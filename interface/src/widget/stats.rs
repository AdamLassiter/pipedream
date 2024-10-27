use log::debug;
use ratatui::{prelude::*, widgets::Paragraph};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use crate::Renderable;
use pipedream_domain::stats::Stats;

impl Renderable for Stats {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        debug!(target:"Interface/Stats/Render", "{:?} at {:?}", self, area);

        let text = self
            .resources
            .iter()
            .map(|(res, amt)| format!("<{} {} {}>", res.style(), amt, res))
            .collect::<Vec<_>>();

        Paragraph::new(
            text.iter()
                .flat_map(|description| {
                    compile::<RatatuiTextGenerator>(description)
                        .into_iter()
                        .flat_map(|text| text.lines)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
        .alignment(Alignment::Center)
        .render(area, buf);
    }
}

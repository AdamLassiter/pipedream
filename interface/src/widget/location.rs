use pipedream_domain::location::LocationStack;
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    symbols,
    widgets::{Tabs, Widget},
};

use crate::Renderable;

impl Renderable for LocationStack {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Tabs::new(self.0.iter().map(|loc| loc.0.clone()))
            .divider(symbols::DOT)
            .highlight_style(Style::default())
            .render(area, buf);
    }
}

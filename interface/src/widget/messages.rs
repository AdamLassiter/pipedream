use pipedream_domain::message::{Message, MessageLog};
use ratatui::prelude::Buffer;
use ratatui::prelude::Rect;
use ratatui::symbols::border;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::{Paragraph, Widget};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use crate::Renderable;

impl Renderable for MessageLog {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let messages_area = block.inner(area);

        let messages_lines = self
            .0
            .iter()
            .flat_map(
                |Message {
                     message,
                     timestamp: _,
                 }| {
                    compile::<RatatuiTextGenerator>(message)
                        .into_iter()
                        .flat_map(|text| text.lines)
                        .collect::<Vec<_>>()
                },
            )
            .collect::<Vec<_>>();
        Widget::render(Paragraph::new(messages_lines), messages_area, buf);

        block.render(area, buf);
    }
}

use ratatui::{prelude::*, widgets::*};
use serde_derive::Serialize;

use super::description::Description;

#[derive(Debug, Clone, Serialize)]
pub struct Scene {
    pub descriptions: Vec<Description>,
}

impl Widget for &Scene {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let scene = Text::from(
            self.descriptions
                .iter()
                .map(|Description(_, line)| Line::from(vec![line.into()]))
                .collect::<Vec<_>>(),
        );
        Widget::render(Paragraph::new(scene), area, buf);
    }
}

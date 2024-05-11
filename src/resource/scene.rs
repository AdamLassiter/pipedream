use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};

use super::description::Description;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub descriptions: Vec<Description>,
}

impl Widget for &Scene {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let scene = Text::from(
            self.descriptions
                .iter()
                .map(|Description { descriptor, .. }| Line::from(vec![descriptor.into()]))
                .collect::<Vec<_>>(),
        );
        Widget::render(Paragraph::new(scene), area, buf);
    }
}

use ratatui::{prelude::*, widgets::*};

use crate::resource::{description::Description, scene::Scene};

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

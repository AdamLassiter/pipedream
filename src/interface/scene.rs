use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone)]
pub struct Scene(pub Vec<String>);

impl Widget for &Scene {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let scene = Text::from(
            self.0
                .iter()
                .map(|line| Line::from(vec![line.into()]))
                .collect::<Vec<_>>(),
        );
        Widget::render(Paragraph::new(scene), area, buf);
    }
}

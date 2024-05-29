use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, Widget},
};

use crate::resource::tag::Tags;

impl Widget for &Tags {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(
            List::new(
                self.iter()
                    .map(|(key, val)| format!("{:?}/{:?}", key.0, val)),
            ),
            area,
            buf,
        );
    }
}

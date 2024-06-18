use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, Widget},
};

use crate::resource::core::tag::Tags;

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

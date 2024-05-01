use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone)]
pub struct Options {
    pub options: Vec<String>,
    pub cursor: usize,
}

impl Options {
    fn cursor_down(&mut self) {
        if self.options.len() > 0 {
            self.cursor = self
                .cursor
                .saturating_add(1)
                .clamp(0, self.options.len() - 1);
        }
    }

    fn cursor_up(&mut self) {
        if self.options.len() > 0 {
            self.cursor = self
                .cursor
                .saturating_sub(1)
                .clamp(0, self.options.len() - 1);
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('w') | KeyCode::Up => self.cursor_up(),
            KeyCode::Char('s') | KeyCode::Down => self.cursor_down(),
            _ => {}
        }
    }
}

impl Widget for &Options {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let options = Text::from(
            self.options
                .iter()
                .map(|option| Line::from(vec![option.into()]))
                .collect::<Vec<_>>(),
        );
        let mut state = ListState::default().with_selected(Some(self.cursor));
        let opts_list = List::new(options).highlight_symbol(">> ");
        StatefulWidget::render(opts_list, area, buf, &mut state);
    }
}

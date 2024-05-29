use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Text},
    widgets::{List, ListState, StatefulWidget, Widget},
};

use crate::resource::{
    choice::{Choice, Choices},
    description::Description,
    transition::Transition,
};

impl Choices {
    pub(crate) fn cursor_down(&mut self) {
        if !self.choices.is_empty() {
            self.cursor = self
                .cursor
                .saturating_add(1)
                .clamp(0, self.choices.len() - 1);
        }
    }

    pub(crate) fn cursor_up(&mut self) {
        if !self.choices.is_empty() {
            self.cursor = self
                .cursor
                .saturating_sub(1)
                .clamp(0, self.choices.len() - 1);
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('w') | KeyCode::Up => self.cursor_up(),
            KeyCode::Char('s') | KeyCode::Down => self.cursor_down(),
            _ => {}
        }
    }

    pub fn current_transition(&self) -> Transition {
        self.choices.get(self.cursor).unwrap().effect.clone()
    }
}

impl Widget for &Choices {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let options = Text::from(
            self.choices
                .iter()
                .map(
                    |Choice {
                         description: Description { descriptor, .. },
                         ..
                     }| { Line::from(vec![descriptor.into()]) },
                )
                .collect::<Vec<_>>(),
        );
        let mut state = ListState::default().with_selected(Some(self.cursor));
        let opts_list = List::new(options).highlight_symbol(">> ");
        StatefulWidget::render(opts_list, area, buf, &mut state);
    }
}

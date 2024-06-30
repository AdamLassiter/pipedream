use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{List, ListState, StatefulWidget, Widget},
};

use crate::engine::core::{
    choice::{Choice, ChoiceType, Choices},
    description::Description,
    transition::Transition,
};

impl Choices {
    pub fn cursor_down(&mut self) {
        if let ChoiceType::Manual(choices) = &self.choices
            && !choices.is_empty()
        {
            self.cursor = self.cursor.saturating_add(1).clamp(0, choices.len() - 1);
        }
    }

    pub fn cursor_up(&mut self) {
        if let ChoiceType::Manual(choices) = &self.choices
            && !choices.is_empty()
        {
            self.cursor = self.cursor.saturating_sub(1).clamp(0, choices.len() - 1);
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('w') | KeyCode::Up => self.cursor_up(),
            KeyCode::Char('s') | KeyCode::Down => self.cursor_down(),
            _ => {}
        }
    }

    pub fn current_transition(&self) -> Option<Transition> {
        match &self.choices {
            ChoiceType::Manual(choices) => choices
                .get(self.cursor)
                .filter(|&c| c.selectable)
                .map(|c| c.effect.clone()),
            ChoiceType::Auto(transition) => Some(transition.clone()),
        }
    }
}

impl Widget for &Choices {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match &self.choices {
            ChoiceType::Manual(choices) => {
                let options = Text::from(
                    choices
                        .iter()
                        .map(
                            |Choice {
                                 description: Description { descriptor, .. },
                                 selectable,
                                 ..
                             }| {
                                Line::from(vec![descriptor.into()]).style(if *selectable {
                                    Style::new().white()
                                } else {
                                    Style::new().dark_gray()
                                })
                            },
                        )
                        .collect::<Vec<_>>(),
                );
                let mut state = ListState::default().with_selected(Some(self.cursor));
                let opts_list = List::new(options).highlight_symbol(">> ");
                StatefulWidget::render(opts_list, area, buf, &mut state);
            }
            ChoiceType::Auto(_) => { /* None */ }
        }
    }
}

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};

use crate::resource::{description::Description, transition::SideEffect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choices {
    pub choices: Vec<Choice>,
    #[serde(default = "zero")]
    #[serde(skip_serializing)]
    cursor: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub description: Description,
    pub effect: SideEffect,
}

fn zero() -> usize {
    0
}

impl From<Vec<(Description, SideEffect)>> for Choices {
    fn from(value: Vec<(Description, SideEffect)>) -> Self {
        Choices {
            choices: value
                .into_iter()
                .map(|(description, effect)| Choice {
                    description,
                    effect,
                })
                .collect(),
            cursor: 0,
        }
    }
}

impl Choices {
    fn cursor_down(&mut self) {
        if self.choices.len() > 0 {
            self.cursor = self
                .cursor
                .saturating_add(1)
                .clamp(0, self.choices.len() - 1);
        }
    }

    fn cursor_up(&mut self) {
        if self.choices.len() > 0 {
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

    pub fn current_effect(&self) -> SideEffect {
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

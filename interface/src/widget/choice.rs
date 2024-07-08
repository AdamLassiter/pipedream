use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, ListState, StatefulWidget},
};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use pipedream_engine::core::{
    choice::{Choice, ChoiceType, Choices},
    description::Description,
    transition::Transition,
};

use crate::{Controllable, Renderable};

impl Controllable for Choices {
    fn cursor_down(&mut self) {
        if let ChoiceType::Manual(choices) = &self.choices
            && !choices.is_empty()
        {
            self.cursor = self.cursor.saturating_add(1).clamp(0, choices.len() - 1);
        }
    }

    fn cursor_up(&mut self) {
        if let ChoiceType::Manual(choices) = &self.choices
            && !choices.is_empty()
        {
            self.cursor = self.cursor.saturating_sub(1).clamp(0, choices.len() - 1);
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('w') | KeyCode::Up => self.cursor_up(),
            KeyCode::Char('s') | KeyCode::Down => self.cursor_down(),
            _ => {}
        }
    }

    fn current_choice(&self) -> Option<Choice> {
        match &self.choices {
            ChoiceType::Manual(choices) => choices.get(self.cursor).cloned(),
            ChoiceType::Auto(..) => None,
        }
    }

    fn current_transition(&self) -> Option<Transition> {
        match &self.choices {
            ChoiceType::Manual(choices) => choices
                .get(self.cursor)
                .filter(|&c| c.selectable)
                .map(|c| c.effect.clone()),
            ChoiceType::Auto(transition, _) => Some(transition.clone()),
        }
    }
}

impl Renderable for Choices {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        match &self.choices {
            ChoiceType::Manual(choices) => {
                let options = choices
                    .iter()
                    .map(|choice| {
                        let Choice {
                            description,
                            selectable,
                            ..
                        } = choice;
                        if *selectable {
                            description.clone()
                        } else {
                            Description {
                                descriptor: format!("<d {}>", description.descriptor),
                                predicate: description.predicate.clone(),
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                // debug!(target:"Render/Choices", "{:?}", options);

                let options = options
                    .iter()
                    .map(|Description { descriptor, .. }| {
                        compile::<RatatuiTextGenerator>(descriptor.as_str())
                            .expect("Failed to compile tui text markup")
                    })
                    .collect::<Vec<_>>();

                let mut state = ListState::default().with_selected(Some(self.cursor));
                let opts_list = List::new(options).highlight_symbol(">> ");

                StatefulWidget::render(opts_list, area, buf, &mut state);
            }
            ChoiceType::Auto(..) => { /* None */ }
        }
    }
}

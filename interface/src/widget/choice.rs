use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, Borders, List, ListState, Padding, Paragraph, StatefulWidget, Widget,
    },
};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use pipedream_engine::core::{
    choice::{Choice, ChoiceType, Choices},
    transition::Transition,
};

use crate::{
    image::{AsciiOptions, ImageConverter, ToAsciiArt},
    Controllable, Renderable,
};

fn cursor_up(this: &mut Choices) {
    if let ChoiceType::Manual(choices) = &this.choices
        && !choices.is_empty()
    {
        this.cursor = this.cursor.saturating_add(1).clamp(0, choices.len() - 1);
    }
}

fn cursor_down(this: &mut Choices) {
    if let ChoiceType::Manual(choices) = &this.choices
        && !choices.is_empty()
    {
        this.cursor = this.cursor.saturating_sub(1).clamp(0, choices.len() - 1);
    }
}

fn cursor_left(_this: &mut Choices) {}

fn cursor_right(_this: &mut Choices) {}

impl Controllable for Choices {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('w') | KeyCode::Up => cursor_up(self),
            KeyCode::Char('s') | KeyCode::Down => cursor_down(self),
            KeyCode::Char('a') | KeyCode::Left => cursor_left(self),
            KeyCode::Char('d') | KeyCode::Right => cursor_right(self),
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

impl Renderable for Choice {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.image.is_none() && self.details.is_empty() && self.cost.is_none() {
            return;
        };

        let details_size_hint = self.details.len() as u16;
        let ascii_size_hint = if details_size_hint > 0 { 16 } else { 32 } as u16;

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .padding(Padding::uniform(1));

        let [ascii_area, details_area] = Layout::vertical([
            Constraint::Min(ascii_size_hint),
            Constraint::Fill(details_size_hint),
        ])
        .areas(block.inner(area));

        let padded_summary = format!(" {} ", self.summary);
        let mut title_text = compile::<RatatuiTextGenerator>(&padded_summary)
            .expect("Failed to compile tui text markup for summaries");
        if let Some(title_line) = title_text.lines.pop() {
            block = block.title(Title {
                content: title_line.bold(),
                alignment: Some(Alignment::Left),
                position: Some(Position::Top),
            });
        }

        let padded_cost; // Must live long enough
        if let Some(cost) = &self.cost {
            padded_cost = format!(" {} ", cost);
            let mut cost_lines = compile::<RatatuiTextGenerator>(&padded_cost)
                .expect("Failed to compile tui text markup for cost")
                .lines;
            if let Some(cost_line) = cost_lines.pop() {
                block = block.title(Title {
                    content: cost_line,
                    alignment: Some(Alignment::Right),
                    position: Some(Position::Bottom),
                });
            }
        }

        if let Some(image) = &self.image {
            let image = ImageConverter::from(&PathBuf::from(image));
            let ascii_text = image.to_ascii_art(Some(AsciiOptions {
                height: ascii_area.height,
                width: ascii_area.height,
                ..Default::default()
            }));
            Paragraph::new(ascii_text)
                .alignment(Alignment::Center)
                .render(ascii_area, buf);

            let details_lines = self
                .details
                .iter()
                .flat_map(|details| {
                    compile::<RatatuiTextGenerator>(details)
                        .into_iter()
                        .flat_map(|text| text.lines)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Paragraph::new(details_lines)
                .alignment(Alignment::Center)
                .render(details_area, buf);
        }

        block.render(area, buf);
    }
}

impl Renderable for (&[Choice], usize) {
    fn render(&self, summary_area: Rect, buf: &mut Buffer) {
        let (choices, cursor) = self;
        let mut state = ListState::default().with_selected(Some(*cursor));

        let summary_descriptions = choices
            .iter()
            .map(|choice| {
                let Choice {
                    summary,
                    selectable,
                    ..
                } = choice;
                let description = if let Some(pred) = &choice.predicate {
                    format!("{} [{}]", summary, pred)
                } else {
                    summary.to_string()
                };
                if *selectable {
                    description
                } else {
                    format!("<d {}>", description)
                }
            })
            .collect::<Vec<_>>();

        let summary_text = summary_descriptions
            .iter()
            .map(|description| {
                compile::<RatatuiTextGenerator>(description)
                    .expect("Failed to compile tui text markup for summaries")
            })
            .collect::<Vec<_>>();

        let summary_list = List::new(summary_text)
            .direction(ratatui::widgets::ListDirection::BottomToTop)
            .highlight_symbol(">> ");
        StatefulWidget::render(summary_list, summary_area, buf, &mut state);
    }
}

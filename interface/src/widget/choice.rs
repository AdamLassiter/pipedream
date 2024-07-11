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

use crate::{Controllable, Renderable};

use super::image::{AsciiOptions, ImageConverter, ToAsciiArt};

fn cursor_up(this: &mut Choices) {
    if let ChoiceType::Manual(choices) = &this.choices
        && !choices.is_empty()
    {
        this.cursor = this.cursor.saturating_sub(1).clamp(0, choices.len() - 1);
    }
}

fn cursor_down(this: &mut Choices) {
    if let ChoiceType::Manual(choices) = &this.choices
        && !choices.is_empty()
    {
        this.cursor = this.cursor.saturating_add(1).clamp(0, choices.len() - 1);
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

impl Renderable for Choices {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        match &self.choices {
            ChoiceType::Manual(choices) => {
                let [summary_area, details_area] =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

                render_choices(self.cursor, choices, summary_area, buf);

                render_choice_details(self.cursor, choices, details_area, buf);
            }
            ChoiceType::Auto(..) => { /* None */ }
        }
    }
}

fn render_choice_details(cursor: usize, choices: &[Choice], area: Rect, buf: &mut Buffer) {
    if let Some(selected) = choices.get(cursor) {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .padding(Padding::uniform(1));
        let [ascii_area, details_area] =
            Layout::vertical([Constraint::Min(16), Constraint::Fill(1)]).areas(block.inner(area));

        let padded_summary = format!(" {} ", selected.summary);
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
        if let Some(cost) = &selected.cost {
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

        let image = ImageConverter::from(&PathBuf::from("resources/tile009.png"));
        let ascii_text = image.to_ascii_art(Some(AsciiOptions::new(32, 16, 1.0)));
        Paragraph::new(ascii_text)
            .alignment(Alignment::Center)
            .render(ascii_area, buf);

        let details_lines = selected
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

        block.render(area, buf);
    }
}

fn render_choices(cursor: usize, choices: &[Choice], summary_area: Rect, buf: &mut Buffer) {
    let mut state = ListState::default().with_selected(Some(cursor));

    let summary_descriptions = choices
        .iter()
        .map(|choice| {
            let Choice {
                summary: description,
                selectable,
                ..
            } = choice;
            if *selectable {
                description.clone()
            } else {
                format!("<d {}>", description)
            }
        })
        .collect::<Vec<_>>();

    // debug!(target:"Render/Choices", "{:?}", options);

    let summary_text = summary_descriptions
        .iter()
        .map(|description| {
            compile::<RatatuiTextGenerator>(description)
                .expect("Failed to compile tui text markup for summaries")
        })
        .collect::<Vec<_>>();

    let summary_list = List::new(summary_text).highlight_symbol(">> ");
    StatefulWidget::render(summary_list, summary_area, buf, &mut state);
}

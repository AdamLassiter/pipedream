use std::ops::{Deref, DerefMut};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, Borders, Clear, Paragraph, Widget,
    },
};
use tui_markup::{compile, generator::RatatuiTextGenerator};

use pipedream_engine::{
    core::{
        choice::{Choice, ChoiceType, Choices},
        transition::Transition,
    },
    log::debug,
};

use crate::{Controllable, Renderable};

pub struct CombatChoices(pub Choices);

impl Deref for CombatChoices {
    type Target = Choices;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CombatChoices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct CombatChoice(pub Choice);

impl Deref for CombatChoice {
    type Target = Choice;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CombatChoice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn cursor_left(this: &mut Choices) {
    if let ChoiceType::Manual(choices) = &this.choices
        && !choices.is_empty()
    {
        this.cursor = this.cursor.saturating_sub(1).clamp(0, choices.len() - 1);
    }
}

fn cursor_right(this: &mut Choices) {
    if let ChoiceType::Manual(choices) = &this.choices
        && !choices.is_empty()
    {
        this.cursor = this.cursor.saturating_add(1).clamp(0, choices.len() - 1);
    }
}

impl Controllable for CombatChoices {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
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

impl Renderable for CombatChoice {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        if self.image.is_none() && self.details.is_empty() && self.cost.is_none() {
            return;
        };

        let details_size_hint = self.details.len() as u16;
        let ascii_size_hint = if details_size_hint > 0 { 32 + 2 } else { 0 } as u16;

        if self.image.is_none() && self.details.is_empty() && self.cost.is_none() {
            return;
        };

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);
        Clear.render(block.inner(area), buf);

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
            image.render(ascii_area, buf);
        }

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
        block.render(area, buf);
    }
}

impl Renderable for CombatChoices {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        debug!(target:"Render/Choice", "render choices {:?} at area {:?}", self.0, area);
        let Choices { choices, cursor } = &self.0;

        if let ChoiceType::Manual(choices) = choices {
            let card_size_hint = 32 + 2;
            let carousel_len = choices.len();
            let space_between = (area.width - card_size_hint) / carousel_len as u16;

            let idx_starts = (0..carousel_len)
                .map(|idx| (idx, ((idx as f32 + 0.5) * space_between as f32) as u16));
            let [initial_card, _] =
                Layout::horizontal([Constraint::Length(card_size_hint), Constraint::Fill(1)])
                    .areas(area);
            let mut idx_areas = idx_starts
                .map(|(idx, starts)| {
                    (
                        idx,
                        Rect {
                            x: starts,
                            y: initial_card.y - (if idx == *cursor { 1 } else { 0 }),
                            ..initial_card
                        },
                    )
                })
                .collect::<Vec<_>>();

            let (left, rest) = idx_areas.split_at_mut(*cursor);
            let (current, right) = rest.split_at_mut(1);

            // Render rhs of hand back-to-front
            right.reverse();

            // Render currently-selected last
            let idx_areas = left.iter().chain(right.iter()).chain(current.iter());

            idx_areas.for_each(|(idx, area)| {
                CombatChoice(choices[*idx].clone()).render(*area, buf);
            });
        }
    }
}

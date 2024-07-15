use crate::{component::Component, Handler, Renderable};
use crossterm::event::KeyEvent;
use pipedream_bichannel::Bichannel;
use pipedream_engine::core::{
    choice::{ChoiceType, Choices},
    commands::{EngineCommand, UiCommand},
};
use ratatui::prelude::*;

use super::scene_and_choices::SceneAndChoicesHandler;

pub struct CombatComponent {
    inner: SceneAndChoicesHandler,
}

impl CombatComponent {
    pub fn new(channel: Bichannel<EngineCommand, UiCommand>) -> Self {
        Self {
            inner: SceneAndChoicesHandler::new(channel),
        }
    }
}

impl Handler for CombatComponent {
    fn handle_tick_event(&mut self) -> bool {
        self.inner.handle_tick_event()
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.inner.handle_key_event(key_event)
    }
}

impl Renderable for CombatComponent {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let stats_size_hint = 5_u16;
        let scene_size_hint = self
            .inner
            .scene
            .as_ref()
            .map(|s| s.descriptions.len())
            .unwrap_or(0) as u16;
        let choices_size_hint = match self.inner.options.as_ref() {
            Some(Choices {
                choices: ChoiceType::Manual(c),
                ..
            }) => c.len(),
            _ => 0,
        } as u16;
        let details_size_hint = self
            .inner
            .options
            .as_ref()
            .and_then(|x| match x {
                Choices {
                    choices: ChoiceType::Manual(choices),
                    cursor,
                } => choices
                    .get(*cursor)
                    .map(|choice| choice.details.len()),
                _ => None,
            })
            .unwrap_or(0) as u16;

        let [description_area, details_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(details_size_hint)])
                .areas(area);
        let [stats_area, scene_area, choices_area] = Layout::vertical([
            Constraint::Length(stats_size_hint),
            Constraint::Min(scene_size_hint),
            Constraint::Min(choices_size_hint),
        ])
        .areas(description_area);

        if let Some(scene) = self.inner.scene.as_ref() {
            scene.render(scene_area, buf);
        }
        if let Some(tags) = self.inner.tags.as_ref() {
            tags.render(stats_area, buf);
        }
        if let Some(Choices {
            choices: ChoiceType::Manual(choices),
            cursor,
        }) = self.inner.options.as_ref()
        {
            (choices.as_slice(), *cursor).render(choices_area, buf);

            if let Some(selected) = choices.get(*cursor) {
                selected.render(details_area, buf);
            }
        }
    }
}

impl Component for CombatComponent {}

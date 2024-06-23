use std::time::Duration;

use crate::{
    interface::{handler::Handler, Component},
    resource::core::{
        choice::{ChoiceType, Choices},
        commands::{EngineCommand, UiCommand},
        scene::Scene,
    },
};
use bichannel::Channel;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

pub struct Campaign {
    scene: Option<Scene>,
    options: Option<Choices>,
}

impl Campaign {
    pub fn new() -> Self {
        Self {
            scene: None,
            options: None,
        }
    }

    fn make_choice(&mut self, channel: &Channel<EngineCommand, UiCommand>) {
        let options = self.options.take();
        if let Some(options) = options {
            if let Some(transition) = options.current_transition() {
                channel
                    .send(EngineCommand::RespondWithChoice(transition))
                    .unwrap();
            }
        }
    }
}

impl Default for Campaign {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Campaign {}

impl Handler for Campaign {
    fn handle_tick_event(&mut self, channel: &Channel<EngineCommand, UiCommand>) {
        if event::poll(Duration::from_millis(0)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event, channel)
                }
                _ => {}
            };
        }
        while let Ok(ev) = channel.try_recv() {
            match ev {
                UiCommand::ShowScene(scen) => self.scene = Some(scen),
                UiCommand::ShowChoices(opts) => self.options = Some(opts),
            }
        }
    }

    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        channel: &Channel<EngineCommand, UiCommand>,
    ) {
        match key_event.code {
            KeyCode::Char('x') | KeyCode::Enter => self.make_choice(channel),
            _ => {}
        }
        if let Some(options) = self.options.as_mut() {
            options.handle_key_event(key_event);
        }
    }

    fn handle_render(&self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf)
    }
}

impl Widget for &Campaign {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = |bottom: Option<usize>| {
            Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(bottom.unwrap_or(0) as u16),
            ])
        };

        let [description_area, choices_area] =
            vertical(self.options.as_ref().map(|x| match &x.choices {
                ChoiceType::Manual(choices) => choices.len(),
                ChoiceType::Auto(_) => 0,
            }))
            .areas(area);

        if let Some(scene) = self.scene.as_ref() {
            scene.render(description_area, buf);
        }

        if let Some(options) = self.options.as_ref() {
            options.render(choices_area, buf);
        }
    }
}

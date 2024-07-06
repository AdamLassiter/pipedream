use std::thread;

use crate::{
    engine::core::{
        choice::{ChoiceType, Choices},
        commands::{EngineCommand, UiCommand},
        scene::Scene,
        tag::Tags,
    },
    interface::{handler::Handler, Component},
};
use bichannel::Channel;
use crossterm::event::{KeyCode, KeyEvent};
use log::debug;
use ratatui::prelude::*;

pub struct CampaignHandler {
    scene: Option<Scene>,
    options: Option<Choices>,
    tags: Option<Tags>,
}

impl CampaignHandler {
    pub fn new() -> Self {
        Self {
            scene: None,
            options: None,
            tags: None,
        }
    }

    fn make_choice(&mut self, channel: &Channel<EngineCommand, UiCommand>) {
        if let Some(options) = self.options.as_ref() {
            if let Some(current) = options.current_choice() {
                debug!(target: "Interface/Choice", "{:?}", current);
            }
            if let Some(transition) = options.current_transition() {
                let _ = self.options.take();
                channel
                    .send(EngineCommand::RespondWithChoice(transition))
                    .expect("Broken channel while responding with choice");
            }
        }
    }
}

impl Default for CampaignHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for CampaignHandler {}

impl Handler for CampaignHandler {
    fn handle_tick_event(&mut self, channel: &Channel<EngineCommand, UiCommand>) {
        if let Some(Choices {
            choices: ChoiceType::Auto(_, duration),
            ..
        }) = self.options.as_ref()
        {
            thread::sleep(*duration);
            self.make_choice(channel);
        }

        while let Ok(ev) = channel.try_recv() {
            match ev {
                UiCommand::ShowScene(scen) => self.scene = Some(scen),
                UiCommand::ShowChoices(opts) => self.options = Some(opts),
                UiCommand::ShowTags(tags) => self.tags = Some(tags),
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
            _ => {
                if let Some(options) = self.options.as_mut() {
                    options.handle_key_event(key_event);
                }
            }
        }
    }

    fn handle_render(&self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf)
    }
}

impl Widget for &CampaignHandler {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let choices_len = self
            .options
            .as_ref()
            .map(|x| match &x.choices {
                ChoiceType::Manual(choices) => choices.len(),
                ChoiceType::Auto(..) => 0,
            })
            .unwrap_or(0) as u16;

        let [description_area, choices_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(choices_len)]).areas(area);

        let [scene_area, resources_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(description_area);

        if let Some(scene) = self.scene.as_ref() {
            scene.render(scene_area, buf);
        }

        if let Some(tags) = self.tags.as_ref() {
            tags.render(resources_area, buf);
        }

        if let Some(options) = self.options.as_ref() {
            options.render(choices_area, buf);
        }
    }
}

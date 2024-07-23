mod campaign_scene;
mod combat_scene;

use std::time::Instant;

use crate::{Controllable, Handler, Renderable, TickResult};
use crossterm::event::{KeyCode, KeyEvent};
use pipedream_bichannel::Bichannel;
use pipedream_engine::{
    core::{
        choice::{ChoiceType, Choices},
        command::{EngineCommand, UiCommand, UiMode},
        scene::Scene,
        tags::Tags,
    },
    log::debug,
};
use ratatui::prelude::*;

use super::Component;

pub struct SceneComponent {
    pub channel: Bichannel<EngineCommand, UiCommand>,
    pub scene: Option<Scene>,
    pub options: Option<Choices>,
    pub tags: Option<Tags>,
    pub wake_time: Option<Instant>,
    ui_mode: UiMode,
}

impl SceneComponent {
    pub fn new(channel: Bichannel<EngineCommand, UiCommand>) -> Self {
        Self {
            channel,
            scene: None,
            options: None,
            tags: None,
            wake_time: None,
            ui_mode: UiMode::Campaign,
        }
    }

    pub fn make_choice(&mut self) {
        if let Some(options) = self.options.as_ref() {
            if let Some(current) = options.current_choice() {
                debug!(target:"Interface/Choice", "{:?}", current);
            }
            if let Some(transition) = options.current_transition() {
                let _ = self.options.take();
                self.channel
                    .send(EngineCommand::RespondWithChoice(transition))
                    .expect("Broken channel while responding with choice");
            }
        }
    }
}

impl Handler for SceneComponent {
    fn handle_tick_event(&mut self) -> TickResult {
        let mut should_redraw = false;

        if let Some(Choices {
            choices: ChoiceType::Auto(_, duration),
            ..
        }) = self.options.as_ref()
        {
            self.wake_time.get_or_insert(
                Instant::now()
                    .checked_add(*duration)
                    .expect("Failed to compute wake time"),
            );
        }
        if let Some(wake_time) = self.wake_time
            && Instant::now() >= wake_time
        {
            self.wake_time.take();
            self.make_choice();
            should_redraw = true;
        }

        while let Ok(ev) = self.channel.try_recv() {
            match ev {
                UiCommand::ShowScene(scen) => self.scene = Some(scen),
                UiCommand::ShowChoices(opts) => self.options = Some(opts),
                UiCommand::ShowTags(tags) => self.tags = Some(tags),
                UiCommand::ChangeMode(mode) => {
                    self.ui_mode = mode;
                }
            }
            should_redraw = true;
        }

        TickResult { should_redraw }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('x') | KeyCode::Enter => self.make_choice(),
            _ => {
                if let Some(options) = self.options.as_mut() {
                    options.handle_key_event(key_event);
                }
            }
        }
    }
}

impl Renderable for SceneComponent {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        match self.ui_mode {
            UiMode::Campaign => self.render_campaign(area, buf),
            UiMode::Combat => self.render_combat(area, buf),
        }
    }
}

impl Component for SceneComponent {}

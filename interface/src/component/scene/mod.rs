mod campaign_scene;
mod combat_scene;

use std::time::Instant;

use bichannel::Bichannel;
use crossterm::event::{KeyCode, KeyEvent};
use log::debug;
use ratatui::prelude::*;

use super::Component;
use crate::{widget::choice::ChoicesWidget, Handler, Renderable, TickResult};
use pipedream_domain::{choice::Choices, image::Image};
use pipedream_engine::{
    command::{EngineCommand, UiCommand, UiMode},
    scene::Scene,
};

pub struct SceneComponent {
    ui_mode: UiMode,
    pub channel: Bichannel<EngineCommand, UiCommand>,
    pub scene: Option<Scene>,
    pub choices: Option<ChoicesWidget>,
    pub player_image: Option<Image>,
    pub enemy_image: Option<Image>,
    pub wake_time: Option<Instant>,
}

impl SceneComponent {
    pub fn new(channel: Bichannel<EngineCommand, UiCommand>) -> Self {
        Self {
            channel,
            scene: None,
            choices: None,
            player_image: None,
            enemy_image: None,
            wake_time: None,
            ui_mode: UiMode::Campaign,
        }
    }

    pub fn make_choice(&mut self) {
        if let Some(options) = self.choices.as_mut() {
            let widget = options.controllable();
            if let Some(current) = widget.current_choice() {
                debug!(target:"Interface/Scene/MakeChoice", "{:?}", current);
            }
            if let Some(transition) = widget.current_transition() {
                let _ = self.choices.take();
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

        if let Some(Choices::Auto(_, duration)) = self.choices.as_ref().map(|c| c.choices()) {
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
                UiCommand::ShowChoices(opts) => {
                    self.choices = Some(ChoicesWidget::new(opts, &self.ui_mode))
                }
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
                if let Some(options) = self.choices.as_mut() {
                    options.controllable().handle_key_event(key_event);
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

use std::time::Instant;

use crate::{Controllable, Handler, Renderable};
use crossterm::event::{KeyCode, KeyEvent};
use pipedream_bichannel::Bichannel;
use pipedream_engine::{
    core::{
        choice::{ChoiceType, Choices},
        commands::{EngineCommand, UiCommand},
        scene::Scene,
        tags::Tags,
    },
    log::debug,
};
use ratatui::prelude::*;

use super::Component;

pub struct InventoryComponent {
    channel: Bichannel<EngineCommand, UiCommand>,
    scene: Option<Scene>,
    options: Option<Choices>,
    tags: Option<Tags>,
    wake_time: Option<Instant>,
}

impl InventoryComponent {
    pub fn new(channel: Bichannel<EngineCommand, UiCommand>) -> Self {
        Self {
            channel,
            scene: None,
            options: None,
            tags: None,
            wake_time: None,
        }
    }

    fn make_choice(&mut self) {
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

impl Handler for InventoryComponent {
    fn handle_tick_event(&mut self) -> bool {
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
            }
            should_redraw = true;
        }

        should_redraw
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

impl Renderable for InventoryComponent {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let [description_area, choices_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);
        let [scene_area, resources_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(description_area);
        let [summary_area, details_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Max(4 + 32 + 16)])
                .areas(choices_area);

        if let Some(scene) = self.scene.as_ref() {
            scene.render(scene_area, buf);
        }
        if let Some(tags) = self.tags.as_ref() {
            tags.render(resources_area, buf);
        }
        if let Some(Choices {
            choices: ChoiceType::Manual(choices),
            cursor,
        }) = self.options.as_ref()
        {
            (choices.as_slice(), *cursor).render(summary_area, buf);

            if let Some(selected) = choices.get(*cursor) {
                selected.render(details_area, buf);
            }
        }
    }
}

impl Component for InventoryComponent {}

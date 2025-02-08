use crossterm::event::KeyEvent;
use log::debug;
use ratatui::prelude::{Buffer, Rect};

use crate::{Handler, Renderable, TickResult, component::Component};
use bichannel::Bichannel;
use pipedream_domain::message::MessageLog;
use pipedream_engine::command::{EngineCommand, UiCommand};

pub struct MessagesComponent {
    channel: Bichannel<EngineCommand, UiCommand>,
    log: MessageLog,
}

impl MessagesComponent {
    pub fn new(channel: Bichannel<EngineCommand, UiCommand>) -> Self {
        Self {
            channel,
            log: MessageLog(vec![]),
        }
    }
}

impl Handler for MessagesComponent {
    fn handle_tick_event(&mut self) -> TickResult {
        let mut should_redraw = false;

        while let Ok(ev) = self.channel.try_recv() {
            debug!(target:"Interface/Event/Message", "{:?}", ev);
            if let UiCommand::ShowMessage(msg) = ev {
                self.log.0.push(msg);
                should_redraw = true;
            }
        }

        TickResult { should_redraw }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            _ => (),
        }
    }
}

impl Renderable for MessagesComponent {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.log.render(area, buf);
    }
}

impl Component for MessagesComponent {}

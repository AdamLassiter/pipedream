use crossterm::event::KeyCode;
use log::LevelFilter;
use pipedream_engine::core::commands::{EngineCommand, UiCommand};
use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Widget,
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetEvent, TuiWidgetState};

use crate::{handler::Handler, Component, Renderable};

pub struct LoggingHandler {
    log: TuiWidgetState,
}

impl LoggingHandler {
    pub fn new() -> Self {
        Self {
            log: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace),
        }
    }
}

impl Default for LoggingHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl Handler for LoggingHandler {
    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        _channel: &bichannel::Channel<EngineCommand, UiCommand>,
    ) {
        match key_event.code {
            KeyCode::Char('w') | KeyCode::Up => self.log.transition(TuiWidgetEvent::UpKey),
            KeyCode::Char('s') | KeyCode::Down => self.log.transition(TuiWidgetEvent::DownKey),
            KeyCode::Char('a') | KeyCode::Left => self.log.transition(TuiWidgetEvent::LeftKey),
            KeyCode::Char('d') | KeyCode::Right => self.log.transition(TuiWidgetEvent::RightKey),
            KeyCode::Char('h') | KeyCode::Delete => self.log.transition(TuiWidgetEvent::HideKey),
            KeyCode::Char('f') | KeyCode::Enter => self.log.transition(TuiWidgetEvent::FocusKey),
            KeyCode::PageUp => self.log.transition(TuiWidgetEvent::PrevPageKey),
            KeyCode::PageDown => self.log.transition(TuiWidgetEvent::NextPageKey),
            _ => (),
        }
    }

    fn handle_tick_event(
        &mut self,
        _channel: &bichannel::Channel<EngineCommand, UiCommand>,
    ) -> bool {
        true // Always re-draw
    }

    fn handle_render(&self, area: Rect, buf: &mut Buffer) {
        self.render(area, buf)
    }
}

impl Component for LoggingHandler {}

impl Renderable for LoggingHandler {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        TuiLoggerSmartWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(' ')
            .output_timestamp(None)
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_target(true)
            .output_file(false)
            .output_line(false)
            .state(&self.log)
            .render(area, buf);
    }
}

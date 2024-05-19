use log::LevelFilter;
use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Widget,
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget, TuiWidgetState};

pub struct Logging {
    log: TuiWidgetState,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            log: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace),
        }
    }
}

impl Widget for &Logging {
    fn render(self, area: Rect, buf: &mut Buffer) {
        TuiLoggerWidget::default()
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

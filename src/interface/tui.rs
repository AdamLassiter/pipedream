use std::{
    io,
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::resource::core::commands::{EngineCommand, UiCommand};
use bichannel::{channel, Channel};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use strum::{Display, EnumCount, FromRepr, VariantArray};

use super::{
    utils,
    handler::{campaign_handler::CampaignHandler, logging_handler::LoggingHandler},
    Component,
};

#[derive(Display, FromRepr, EnumCount, VariantArray, Copy, Clone)]
#[repr(usize)]
enum SelectedTab {
    Campaign = 0,
    Logging = 1,
}
impl SelectedTab {
    fn title(self) -> Line<'static> {
        format!("  {self}  ").fg(Color::Cyan).into()
    }
}

pub struct Tui {
    current_tab: SelectedTab,
    tabs: Vec<Box<dyn Component>>,
    channel: Channel<EngineCommand, UiCommand>,
    exit: bool,
}

impl Tui {
    fn new() -> (Self, Channel<UiCommand, EngineCommand>) {
        let (ui_chan, engine_chan) = channel();

        let this = Self {
            current_tab: SelectedTab::Campaign,
            tabs: vec![
                Box::new(CampaignHandler::new()),
                Box::new(LoggingHandler::new()),
            ],
            channel: ui_chan,
            exit: false,
        };

        (this, engine_chan)
    }

    pub fn spawn() -> (
        Channel<UiCommand, EngineCommand>,
        JoinHandle<io::Result<()>>,
    ) {
        let (mut app, chan) = Tui::new();

        (
            chan,
            thread::spawn(move || {
                let mut terminal = utils::init()?;
                while !app.exit {
                    terminal.draw(|frame| app.render_frame(frame))?;
                    app.handle_events();
                }
                utils::restore()?;
                Ok(())
            }),
        )
    }

    fn exit(&mut self) {
        self.channel.send(EngineCommand::Exit).unwrap();
        self.exit = true;
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) {
        if event::poll(Duration::from_millis(10)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        self.handle_tick_event();
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char('q') => {
                self.current_tab = SelectedTab::from_repr(
                    (self.current_tab as i32 - 1).rem_euclid(SelectedTab::COUNT as i32) as usize,
                )
                .unwrap();
            }
            KeyCode::Char('e') => {
                self.current_tab = SelectedTab::from_repr(
                    (self.current_tab as i32 + 1).rem_euclid(SelectedTab::COUNT as i32) as usize,
                )
                .unwrap();
            }
            _ => {}
        }
        self.tabs
            .iter_mut()
            .for_each(|tab| tab.handle_key_event(key_event, &self.channel));
    }

    fn handle_tick_event(&mut self) {
        self.tabs
            .iter_mut()
            .for_each(|tab| tab.handle_tick_event(&self.channel));
    }

    fn handle_render(&self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" PipeDream ".bold());

        let instructions = Title::from(Line::from(vec![
            " Up ".into(),
            "<W>".blue().bold(),
            " Down ".into(),
            "<S>".blue().bold(),
            " Left ".into(),
            "<A>".blue().bold(),
            " Right ".into(),
            "<D>".blue().bold(),
            " Prev ".into(),
            "<Q>".blue().bold(),
            " Next ".into(),
            "<E>".blue().bold(),
            " Enter ".into(),
            "<X>".blue().bold(),
            " Quit ".into(),
            "<Esc> ".blue().bold(),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .padding(Padding::uniform(1));

        let tabs = Tabs::new(SelectedTab::VARIANTS.iter().map(|&var| var.title()))
            .highlight_style(Color::Blue)
            .select(self.current_tab as usize)
            .padding("", "")
            .divider(" ");

        let vertical = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]);
        let [header_area, inner_area] = vertical.areas(block.inner(area));

        if let Some(tab) = self.tabs.get(self.current_tab as usize) {
            tab.handle_render(inner_area, buf);
        }
        tabs.render(header_area, buf);
        block.render(area, buf);
    }
}

impl Widget for &Tui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.handle_render(area, buf);
    }
}

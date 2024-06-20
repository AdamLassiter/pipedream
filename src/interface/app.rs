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

use super::{
    utils,
    widgets::{campaign::Campaign, logging::Logging},
    Component,
};

pub struct App {
    tabs: Vec<Box<dyn Component>>,
    channel: Channel<EngineCommand, UiCommand>,
    exit: bool,
    log: Logging,
}

impl App {
    fn new() -> (Self, Channel<UiCommand, EngineCommand>) {
        let (ui_chan, engine_chan) = channel();

        let this = Self {
            tabs: vec![Box::new(Campaign::new())],
            channel: ui_chan,
            exit: false,
            log: Logging::new(),
        };

        (this, engine_chan)
    }

    pub fn spawn() -> (
        Channel<UiCommand, EngineCommand>,
        JoinHandle<io::Result<()>>,
    ) {
        let (mut app, chan) = App::new();

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
            KeyCode::Char('q') => todo!(),
            KeyCode::Char('e') => todo!(),
            _ => {}
        }
        if let Some(tab) = self.tabs.get_mut(0) {
            tab.handle_key_event(key_event, &self.channel);
        }
    }

    fn handle_tick_event(&mut self) {
        if let Some(tab) = self.tabs.get_mut(0) {
            tab.handle_tick_event(&self.channel);
        }
    }

    fn handle_render(&self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" PipeDream ".bold());

        let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);

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
            "<e>".blue().bold(),
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

        let [game_area, debug_area] = horizontal.areas(block.inner(area));

        if let Some(tab) = self.tabs.get(0) {
            tab.handle_render(game_area, buf);
        }

        self.log.render(debug_area, buf);

        block.render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.handle_render(area, buf);
    }
}

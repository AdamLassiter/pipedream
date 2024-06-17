use std::{
    io,
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::resource::{
    choice::Choices,
    commands::{EngineCommand, UiCommand},
    scene::Scene,
};
use bichannel::{channel, Channel};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use super::{utils, widgets::logging::Logging};

pub struct App {
    scene: Option<Scene>,
    options: Option<Choices>,
    channel: Channel<EngineCommand, UiCommand>,
    exit: bool,
    log: Logging,
}

impl App {
    fn new() -> (Self, Channel<UiCommand, EngineCommand>) {
        let (ui_chan, engine_chan) = channel();

        let this = App {
            scene: None,
            options: None,
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
                    app.handle_events()?;
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

    fn make_choice(&mut self) {
        let options = self.options.take();
        if let Some(options) = options {
            if let Some(transition) = options.current_transition() {
                self.channel
                    .send(EngineCommand::RespondWithChoice(transition))
                    .unwrap();
            }
        }
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('x') | KeyCode::Enter => self.make_choice(),
            _ => {}
        }
        if let Some(options) = self.options.as_mut() {
            options.handle_key_event(key_event);
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        while let Ok(ev) = self.channel.try_recv() {
            match ev {
                UiCommand::ShowScene(scen) => self.scene = Some(scen),
                UiCommand::ShowChoices(opts) => self.options = Some(opts),
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" PipeDream ".bold());

        let vertical = |bottom: Option<usize>| {
            Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(bottom.unwrap_or(0) as u16),
            ])
        };
        let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);

        let instructions = Title::from(Line::from(vec![
            " Up ".into(),
            "<W>".blue().bold(),
            " Down ".into(),
            "<S>".blue().bold(),
            " Enter ".into(),
            "<X>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
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
        let [description_area, choices_area] =
            vertical(self.options.as_ref().map(|x| x.choices.len())).areas(game_area);

        if let Some(scene) = self.scene.as_ref() {
            scene.render(description_area, buf);
        }

        if let Some(options) = self.options.as_ref() {
            options.render(choices_area, buf);
        }

        self.log.render(debug_area, buf);

        block.render(area, buf);
    }
}

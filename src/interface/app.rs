use std::{
    io,
    thread::{self, JoinHandle},
    time::Duration,
};

use bichannel::{channel, Channel};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use super::{
    commands::{EngineCommand, UiCommand},
    options::Options,
    scene::Scene,
    tui,
};

#[derive(Debug)]
pub struct App {
    scene: Option<Scene>,
    options: Option<Options>,
    channel: Channel<UiCommand, EngineCommand>,
    exit: bool,
}

impl App {
    pub fn new() -> (Self, Channel<EngineCommand, UiCommand>) {
        let (ui_chan, engine_chan) = channel();

        let this = App {
            scene: None,
            options: None,
            channel: ui_chan,
            exit: false,
        };

        (this, engine_chan)
    }

    pub fn spawn() -> (
        Channel<EngineCommand, UiCommand>,
        JoinHandle<io::Result<()>>,
    ) {
        let (mut app, chan) = App::new();

        (
            chan,
            thread::spawn(move || {
                let mut terminal = tui::init()?;
                while !app.exit {
                    terminal.draw(|frame| app.render_frame(frame))?;
                    app.handle_events()?;
                }
                tui::restore()?;
                Ok(())
            }),
        )
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn make_choice(&mut self) {
        let options = self.options.take();
        if let Some(options) = options {
            self.channel.send(UiCommand::Choice(options)).unwrap();
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
        while let Some(ev) = self.channel.try_recv().ok() {
            match ev {
                EngineCommand::NewScene(scen) => self.scene = Some(scen),
                EngineCommand::NeedChoice(opts) => self.options = Some(opts),
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" PipeDream ".bold());

        let vertical = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(self.options.as_ref().map(|x| x.options.len()).unwrap_or(0) as u16),
        ]);

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

        let [scene_area, options_area] = vertical.areas(block.inner(area));

        if let Some(scene) = self.scene.as_ref() {
            scene.render(scene_area, buf);
        }

        if let Some(options) = self.options.as_ref() {
            options.render(options_area, buf);
        }

        block.render(area, buf);
    }
}

use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use bichannel::{Bichannel, BichannelMonitor};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use pipedream_engine::command::{EngineCommand, UiCommand};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{block::Position, Borders, Tabs, Widget},
    Frame,
};
use ratatui::{
    style::{Color, Stylize},
    symbols::border,
    widgets::block::{Block, Padding, Title},
};
use strum::{Display, EnumCount, FromRepr, VariantArray};

use crate::{
    component::{
        inventory::InventoryComponent, logging::LoggingComponent, scene::SceneComponent, Component,
    },
    log_utils,
    widget::instructions::instructions,
};

#[derive(Display, FromRepr, EnumCount, VariantArray, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
enum SelectedTab {
    Campaign = 0,
    Inventory = 1,
    Logging = 2,
}
impl SelectedTab {
    fn title(self) -> Line<'static> {
        format!("  {self}  ").fg(Color::Cyan).into()
    }
}

pub struct Tui {
    current_tab: SelectedTab,
    tabs: Vec<Box<dyn Component>>,
    channel: Bichannel<EngineCommand, UiCommand>,
    should_redraw: bool,
    exit: bool,
}

impl Tui {
    fn new(monitor: &mut BichannelMonitor<EngineCommand, UiCommand>) -> Self {
        let channel = monitor.new_left();

        Self {
            current_tab: SelectedTab::Campaign,
            tabs: vec![
                Box::new(SceneComponent::new(monitor.new_left())),
                Box::new(InventoryComponent::new(monitor.new_left())),
                Box::new(LoggingComponent::new()),
            ],
            channel,
            should_redraw: true,
            exit: false,
        }
    }

    pub fn spawn(monitor: &mut BichannelMonitor<EngineCommand, UiCommand>) -> JoinHandle<()> {
        let mut this = Self::new(monitor);

        thread::spawn(move || {
            let mut terminal = log_utils::init().expect("Failed to init terminal state");
            while !this.exit {
                if this.should_redraw {
                    terminal
                        .draw(|frame| this.render_frame(frame))
                        .expect("Failed to render terminal frame");
                    this.should_redraw = false;
                }
                this.handle_events();
            }
            log_utils::restore().expect("Failed to restore terminal state");
        })
    }

    fn exit(&mut self) {
        self.channel
            .send(EngineCommand::Exit)
            .expect("Broken channel while exiting");
        self.exit = true;
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) {
        while event::poll(Duration::from_millis(10))
            .expect("Event pollng error while handling events")
        {
            match event::read().expect("Event read error while handling events") {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
            self.should_redraw = true;
        }
        self.handle_tick_event();
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Tab => {
                self.current_tab = SelectedTab::from_repr(
                    (self.current_tab as i32 + 1).rem_euclid(SelectedTab::COUNT as i32) as usize,
                )
                .unwrap_or_else(|| {
                    panic!(
                        "Current tab index {:?} outside of bounds of SelectedTab enum repr",
                        self.current_tab as i32 + 1
                    )
                });
            }
            _ => {}
        }

        self.tabs
            .get_mut(self.current_tab as usize)
            .unwrap_or_else(|| panic!("Failed to find current tab {}", self.current_tab))
            .handle_key_event(key_event);
    }

    fn handle_tick_event(&mut self) {
        self.tabs.iter_mut().enumerate().for_each(|(index, tab)| {
            let tick_result = tab.handle_tick_event();
            if self.current_tab as usize == index {
                self.should_redraw |= tick_result.should_redraw;
            }
        });
    }

    fn handle_render(&self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" PipeDream ".bold());
        let instructions = instructions();
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
            .highlight_style(Style::default().fg(Color::White))
            .select(self.current_tab as usize)
            .padding("", "")
            .divider(" ");

        let [_, area, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(160),
            Constraint::Fill(1),
        ])
        .areas(area);
        let [_, area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(50),
            Constraint::Fill(1),
        ])
        .areas(area);

        let [header_area, inner_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(block.inner(area));

        if let Some(tab) = self.tabs.get(self.current_tab as usize) {
            tab.render(inner_area, buf);
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

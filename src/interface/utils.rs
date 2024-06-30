use tui_logger;

use std::{
    io::{self, stdout, Stdout},
    panic,
    thread::{self, JoinHandle},
    time::Duration,
};

use crossterm::{execute, terminal::*};
use ratatui::prelude::*;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let tui = Terminal::new(CrosstermBackend::new(stdout()));

    tui_logger::init_logger(log::LevelFilter::Trace).expect("Failed to initialise logger");
    tui_logger::set_default_level(log::LevelFilter::Trace);

    panic::update_hook(move |prev, info| {
        let _ = restore();
        prev(info);
    });

    tui
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn finish_and_panic_threads(threads: Vec<JoinHandle<io::Result<()>>>) {
    // wait for a thread to finish
    while !(threads.iter().any(|thread| thread.is_finished())) {
        thread::sleep(Duration::from_millis(100))
    }

    // panic first joined errored thread
    threads
        .into_iter()
        .filter(|thread| thread.is_finished())
        .flat_map(|thread| thread.join().err().into_iter())
        .for_each(|err| panic::resume_unwind(err));
}

use log::LevelFilter;
use tui_logger;

use std::{
    io::{self, Stdout, stdout},
    panic,
    thread::{self, JoinHandle},
};

use crossterm::{execute, terminal::*};
use ratatui::prelude::*;

use crate::CULL_POLL_INTERVAL;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let tui = Terminal::new(CrosstermBackend::new(stdout()));

    if true {
        tui_logger::init_logger(LevelFilter::Trace).expect("Failed to initialise logger");
        tui_logger::set_default_level(LevelFilter::Trace);
    }
    // } else {
    //     init_logfile();
    // }

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

pub fn finish_and_panic_threads(threads: Vec<JoinHandle<()>>) {
    // wait for a thread to finish
    while !(threads.iter().any(|thread| thread.is_finished())) {
        thread::sleep(CULL_POLL_INTERVAL)
    }

    // panic first joined errored thread
    threads
        .into_iter()
        .filter(|thread| thread.is_finished())
        .flat_map(|thread| thread.join().err().into_iter())
        .for_each(|err| panic::resume_unwind(err));
}

// fn init_logfile() {
//     use log4rs::{
//         append::file::FileAppender,
//         config::{Appender, Root},
//         encode::pattern::PatternEncoder,
//         Config,
//     };

//     let logfile = FileAppender::builder()
//         .encoder(Box::new(PatternEncoder::new("{l} {t} - {m}\n")))
//         .build("./log")
//         .unwrap();
//     let config = Config::builder()
//         .appender(Appender::builder().build("logfile", Box::new(logfile)))
//         .build(
//             Root::builder()
//                 .appender("logfile")
//                 .build(LevelFilter::Trace),
//         )
//         .unwrap();
//     log4rs::init_config(config).unwrap();
// }

use pipedream::interface::{app::App, commands::EngineCommand, options::Options, scene::Scene};

use std::{io::Result, thread};

fn main() -> Result<()> {
    let (chan, ui_thread) = App::spawn();

    thread::spawn(move || {
        let scene = Scene(vec!["You are in the woods".to_string()]);

        let options = Options {
            options: vec![
                "Go alone".to_string(),
                "Take this".to_string(),
                "[INT 3] Leeroy".to_string(),
            ],
            cursor: 0,
        };

        loop {
            chan.send(EngineCommand::NewScene(scene.clone())).unwrap();
            chan.send(EngineCommand::NeedChoice(options.clone()))
                .unwrap();
            let _ = chan.recv();
        }
    });

    ui_thread.join().unwrap()
}

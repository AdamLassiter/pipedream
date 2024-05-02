use pipedream::{
    interface::{
        app::App,
        commands::{EngineCommand, UiCommand},
        options::Options,
        scene::Scene,
    },
    resource::location::Location,
    statemachine::transition::Transition,
    tagengine::action::Action,
};

use std::{io::Result, thread};

fn main() -> Result<()> {
    let (chan, ui_thread) = App::spawn();

    thread::spawn(move || {
        let options = Options {
            options: vec![
                (
                    "Go alone".to_string(),
                    Transition {
                        next: Location("You are still in the woods".into()),
                        action: Action {},
                    },
                ),
                (
                    "Take this".to_string(),
                    Transition {
                        next: Location("You are in the woods... with a sword!".into()),
                        action: Action {},
                    },
                ),
                (
                    "[INT 3] Leeroy".to_string(),
                    Transition {
                        next: Location("You are dead... in the woods".into()),
                        action: Action {},
                    },
                ),
            ],
            cursor: 0,
        };

        let mut location: Location = Location("You are in the woods".to_string());

        loop {
            let scene = Scene(vec![location.clone().0]);

            chan.send(EngineCommand::NewScene(scene.clone())).unwrap();
            chan.send(EngineCommand::NeedChoice(options.clone()))
                .unwrap();

            match chan.recv().unwrap() {
                UiCommand::Choice(Transition { next, action: _ }) => {
                    location = next;
                }
            }
        }
    });

    ui_thread.join().unwrap()
}

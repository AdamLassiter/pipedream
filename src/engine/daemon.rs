use std::io;
use std::thread;
use std::thread::JoinHandle;

use crate::resource::commands::EngineCommand;
use crate::resource::commands::UiCommand;
use crate::resource::location::Location;
use crate::resource::transition::SideEffect;

use bichannel::Channel;

use super::game::Game;

pub struct Daemon {
    pub game: Game,
    pub channel: Channel<UiCommand, EngineCommand>,
    pub exit: bool,
}

impl Daemon {
    pub fn handle_commands(&mut self) {
        while let Some(ev) = self.channel.try_recv().ok() {
            match ev {
                EngineCommand::Choice(effect) => {
                    self.handle_effect(effect);
                }
                EngineCommand::Exit => self.exit = true,
            }
        }
    }

    fn handle_effect(&mut self, effect: SideEffect) {
        let commands = self.game.handle_effect(effect);
        commands
            .into_iter()
            .for_each(|command| self.channel.send(command).unwrap())
    }

    fn init(&mut self, start: Location) {
        self.handle_effect(SideEffect {
            next: crate::resource::transition::TransitionType::Push(start),
            actions: vec![],
        });
    }

    pub fn spawn(
        game: Game,
        channel: Channel<UiCommand, EngineCommand>,
    ) -> JoinHandle<io::Result<()>> {
        let start = game.start.clone();

        let mut this = Daemon {
            game,
            channel,
            exit: false,
        };

        thread::spawn(move || {
            this.init(start);
            while !this.exit {
                this.handle_commands();
            }
            Ok(())
        })
    }
}

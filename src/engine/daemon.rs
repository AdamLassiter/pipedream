use std::io;
use std::thread;
use std::thread::JoinHandle;

use crate::resource::commands::EngineCommand;
use crate::resource::commands::UiCommand;
use crate::resource::transition::Transition;

use bichannel::Channel;

use super::state_machine::StateMachine;

pub struct Daemon {
    pub machine: StateMachine,
    pub channel: Channel<UiCommand, EngineCommand>,
    pub exit: bool,
}

impl Daemon {
    pub fn handle_commands(&mut self) {
        while let Some(ev) = self.channel.try_recv().ok() {
            match ev {
                EngineCommand::Choice(transition) => {
                    self.handle_transition(transition);
                }
                EngineCommand::Exit => self.exit = true,
            }
        }
    }

    fn handle_transition(&mut self, transition: Transition) {
        let commands = self.machine.handle_transition(transition);
        commands
            .into_iter()
            .for_each(|command| self.channel.send(command).unwrap())
    }

    fn init(&mut self) {
        self.handle_transition(Transition {
            next: self.machine.current.clone(),
            actions: vec![],
        });
    }

    pub fn spawn(
        machine: StateMachine,
        channel: Channel<UiCommand, EngineCommand>,
    ) -> JoinHandle<io::Result<()>> {
        let mut this = Daemon {
            machine,
            channel,
            exit: false,
        };

        thread::spawn(move || {
            this.init();
            while !this.exit {
                this.handle_commands();
            }
            Ok(())
        })
    }
}

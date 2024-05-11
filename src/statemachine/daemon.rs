use std::io;
use std::thread;
use std::thread::JoinHandle;

use crate::resource::commands::EngineCommand;
use crate::resource::commands::UiCommand;
use crate::resource::transition::Transition;
use crate::tagengine::engine::TagEngine;

use bichannel::Channel;

use super::machine::StateMachine;

pub struct Daemon {
    pub machine: StateMachine,
    pub engine: TagEngine,
    pub channel: Channel<EngineCommand, UiCommand>,
    pub exit: bool,
}

impl Daemon {
    pub fn handle_commands(&mut self) {
        while let Some(ev) = self.channel.try_recv().ok() {
            match ev {
                UiCommand::Choice(transition) => {
                    self.handle_transition(transition);
                }
                UiCommand::Exit => self.exit = true,
            }
        }
    }

    fn handle_transition(&mut self, transition: Transition) {
        self.engine.run_actions(transition.actions);
        let commands = self.machine.change_state(transition.next);
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
        engine: TagEngine,
        channel: Channel<EngineCommand, UiCommand>,
    ) -> JoinHandle<io::Result<()>> {
        let mut this = Daemon {
            machine,
            engine,
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

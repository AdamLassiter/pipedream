use log::debug;
use std::{thread, time::Instant};
use std::thread::JoinHandle;

use bichannel::{Bichannel, BichannelMonitor};
use pipedream_engine::{
    command::{EngineCommand, UiCommand},
    effect::Effect,
    state_machine::StateMachine,
};

pub struct GameCoordinator {
    pub states: StateMachine,
    pub channel: Bichannel<UiCommand, EngineCommand>,
    pub exit: bool,
}

impl GameCoordinator {
    fn init(&mut self) {
        self.states
            .next_options()
            .into_iter()
            .for_each(|command| {
                self.channel
                    .send(command)
                    .expect("Broken channel while initialising first options")
            });
    }

    fn handle_commands(&mut self) {
        while let Ok(ev) = self.channel.recv() {
            match ev {
                EngineCommand::RespondWithChoice(effect) => {
                    debug!(target:"Coordinator/Tick", "{:?}", Instant::now());
                    self.handle_effect(effect);
                }
                EngineCommand::Exit => self.exit = true,
            }
        }
    }

    fn handle_effect(&mut self, effect: Effect) {
        let commands = self.states.handle_effect(effect);
        commands.into_iter().for_each(|command| {
            debug!(target:"Coordinator/HandleEffect", "{:?}", command);
            self.channel
                .send(command)
                .expect("Broken channel while handling effect")
        })
    }

    pub fn spawn(
        monitor: &mut BichannelMonitor<EngineCommand, UiCommand>,
        states: StateMachine,
    ) -> JoinHandle<()> {
        let mut this = GameCoordinator {
            states,
            channel: monitor.new_right(),
            exit: false,
        };

        thread::spawn(move || {
            this.init();
            while !this.exit {
                this.handle_commands();
            }
        })
    }
}

use std::thread;
use std::thread::JoinHandle;
use std::time;

use pipedream_bichannel::{Bichannel, BichannelMonitor};
use pipedream_engine::{
    log::debug,
    state::campaign_state_machine::StateMachine,
    {
        command::{EngineCommand, UiCommand},
        effect::Effect,
    },
};

pub struct GameCoordinator {
    pub campaign: StateMachine,
    pub channel: Bichannel<UiCommand, EngineCommand>,
    pub exit: bool,
}

impl GameCoordinator {
    fn init(&mut self) {
        self.campaign
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
                    debug!(target:"Coordinator/Tick", "{:?}", time::Instant::now());
                    self.handle_effect(effect);
                }
                EngineCommand::Exit => self.exit = true,
            }
        }
    }

    fn handle_effect(&mut self, effect: Effect) {
        let commands = self.campaign.handle_effect(effect);
        commands.into_iter().for_each(|command| {
            debug!(target:"Coordinator/HandleEffect", "{:?}", command);
            self.channel
                .send(command)
                .expect("Broken channel while handling effect")
        })
    }

    pub fn spawn(
        monitor: &mut BichannelMonitor<EngineCommand, UiCommand>,
        campaign: StateMachine,
    ) -> JoinHandle<()> {
        let mut this = GameCoordinator {
            campaign,
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

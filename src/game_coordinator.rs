use std::io;
use std::thread;
use std::thread::JoinHandle;
use std::time;

use pipedream_engine::{
    core::{
        commands::{EngineCommand, UiCommand},
        transition::Transition,
    },
    state::campaign_state_machine::CampaignStateMachine,
};

use bichannel::Channel;
use log::debug;

pub struct GameCoordinator {
    pub campaign: CampaignStateMachine,
    pub channel: Channel<UiCommand, EngineCommand>,
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

    fn handle_effect(&mut self, effect: Transition) {
        let commands = self.campaign.handle_effect(effect);
        commands.into_iter().for_each(|command| {
            self.channel
                .send(command)
                .expect("Broken channel while handling effect")
        })
    }

    pub fn spawn(
        campaign: CampaignStateMachine,
        channel: Channel<UiCommand, EngineCommand>,
    ) -> JoinHandle<io::Result<()>> {
        let mut this = GameCoordinator {
            campaign,
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

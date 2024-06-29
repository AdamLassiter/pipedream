use std::io;
use std::thread;
use std::thread::JoinHandle;

use crate::resource::core::transition::TransitionType;
use crate::resource::core::{
    commands::{EngineCommand, UiCommand},
    location::Location,
    transition::Transition,
};

use bichannel::Channel;

use super::campaign_coordinator::CampaignCoordinator;
use super::Coordinator;

pub struct GameCoordinator {
    pub campaign: CampaignCoordinator,
    pub channel: Channel<UiCommand, EngineCommand>,
    pub exit: bool,
}

impl GameCoordinator {
    pub fn handle_commands(&mut self) {
        while let Ok(ev) = self.channel.recv() {
            match ev {
                EngineCommand::RespondWithChoice(effect) => {
                    self.handle_effect(effect);
                }
                EngineCommand::Exit => self.exit = true,
            }
        }
    }

    fn handle_effect(&mut self, effect: Transition) {
        let commands = self.campaign.handle_effect(effect);
        commands
            .into_iter()
            .for_each(|command| self.channel.send(command).expect("Broken channel"))
    }

    fn init(&mut self, start: Location) {
        self.handle_effect(Transition {
            next: TransitionType::Enter(start),
            actions: vec![],
        });
    }

    pub fn spawn(
        game: CampaignCoordinator,
        channel: Channel<UiCommand, EngineCommand>,
    ) -> JoinHandle<io::Result<()>> {
        let start = game.start.clone();

        let mut this = GameCoordinator {
            campaign: game,
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

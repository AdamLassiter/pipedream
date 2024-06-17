use std::io;

use pipedream::{
    engine::{
        coordinator::{campaign_coordinator::CampaignCoordinator, Coordinator},
        game_coordinator::GameCoordinator,
        state_machine::campaign_state_machine::CampaignStateMachine,
        tag_engine::TagEngine,
    },
    interface::{app::App, utils::finish_and_panic_threads},
    resource::{location::Location, world::campaign_world::CampaignWorld},
};

fn main() -> io::Result<()> {
    let (channel, ui_thread) = App::spawn();

    let world = CampaignWorld::generate();
    let tag_engine = TagEngine::generate_campaign();
    let start = Location("woods:entrance".into());

    let state_machine = CampaignStateMachine::new(world);

    let campaign = CampaignCoordinator {
        start,
        tag_engine,
        state_machine,
    };

    {
        campaign.dump();
    }

    let engine_thread = GameCoordinator::spawn(campaign, channel);

    finish_and_panic_threads(vec![ui_thread, engine_thread]);

    Ok(())
}

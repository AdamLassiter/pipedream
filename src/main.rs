use std::io;

use pipedream::{
    engine::{
        core::location::Location,
        state::{
            campaign_state_machine::CampaignStateMachine, campaign_world::CampaignWorld,
            tag_engine::TagEngine,
        },
    },
    game_coordinator::GameCoordinator,
    interface::{tui::Tui, utils::finish_and_panic_threads},
};

fn main() -> io::Result<()> {
    let (channel, ui_thread) = Tui::spawn();

    let world = CampaignWorld::generate();
    let tag_engine = TagEngine::generate_campaign();
    let start = Location("woods:entrance".into());
    let campaign = CampaignStateMachine::new(world, tag_engine, start);

    let engine_thread = GameCoordinator::spawn(campaign, channel);

    finish_and_panic_threads(vec![ui_thread, engine_thread]);

    Ok(())
}

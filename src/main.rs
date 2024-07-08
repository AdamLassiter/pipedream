use std::io;

use pipedream::game_coordinator::GameCoordinator;
use pipedream_engine::{
    core::location::Location,
    state::{
        campaign_state_machine::CampaignStateMachine, campaign_world::CampaignWorld,
        tag_engine::TagEngine,
    },
};
use pipedream_interface::{tui::Tui, utils::finish_and_panic_threads};
use pipedream_prefab::{campaign_state_machine::campaign_exporter, Generatable};

fn main() -> io::Result<()> {
    let (channel, ui_thread) = Tui::spawn();

    let world = CampaignWorld::generate();
    let tag_engine = TagEngine::generate();
    let start = Location("woods:entrance".into());
    let campaign = CampaignStateMachine::new(world, tag_engine, start, campaign_exporter);

    let engine_thread = GameCoordinator::spawn(campaign, channel);

    finish_and_panic_threads(vec![ui_thread, engine_thread]);

    Ok(())
}

use std::io;

use pipedream::game_coordinator::GameCoordinator;
use pipedream_bichannel::BichannelMonitor;
use pipedream_engine::{
    core::location::Location,
    state::{
        campaign_state_machine::CampaignStateMachine, campaign_world::CampaignWorld,
        tag_engine::TagEngine,
    },
};
use pipedream_interface::{log_utils::finish_and_panic_threads, tui::Tui};
use pipedream_prefab::{campaign_state_machine::campaign_exporter, Generatable};

fn main() -> io::Result<()> {
    let (mut monitor, monitor_thread) = BichannelMonitor::spawn();

    let ui_thread = Tui::spawn(&mut monitor);

    let world = CampaignWorld::generate();
    let tag_engine = TagEngine::generate();
    let start = Location("woods:entrance".into());
    let campaign = CampaignStateMachine::new(world, tag_engine, start, campaign_exporter);

    let engine_thread = GameCoordinator::spawn(&mut monitor, campaign);

    finish_and_panic_threads(vec![ui_thread, engine_thread, monitor_thread]);

    Ok(())
}

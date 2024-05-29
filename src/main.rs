use std::io;

use pipedream::{
    engine::{
        campaign::Campaign, daemon::Daemon, static_state_machine::StaticStateMachine,
        tag_engine::TagEngine,
    },
    interface::app::App,
    resource::{location::Location, prefab::campaign_world::CampaignWorld},
};

fn main() -> io::Result<()> {
    let (channel, ui_thread) = App::spawn();

    let world = CampaignWorld::generate_campaign();
    let tag_engine = TagEngine::generate();
    let start = Location("woods:entrance".into());

    let state_machine = StaticStateMachine {
        world,
        current: vec![],
    };

    let game = Campaign {
        start,
        tag_engine,
        state_machine,
    };

    {
        game.dump();
    }

    let engine_thread = Daemon::spawn(game, channel);

    engine_thread.join().unwrap()?;
    ui_thread.join().unwrap()?;
    Ok(())
}

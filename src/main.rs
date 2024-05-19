use std::io;

use pipedream::{
    engine::{
        campaign::Campaign, daemon::Daemon, state_machine::StateMachine, tag_engine::TagEngine,
    },
    interface::app::App,
    resource::{location::Location, world::World},
};

fn main() -> io::Result<()> {
    let (channel, ui_thread) = App::spawn();

    let world = World::generate();
    let tag_engine = TagEngine::generate();
    let start = Location("woods:entrance".into());

    let state_machine = StateMachine {
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

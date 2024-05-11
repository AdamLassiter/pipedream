use std::io;

use pipedream::{
    interface::app::App,
    resource::{location::Location, world::World},
    statemachine::{daemon::Daemon, machine::StateMachine},
    tagengine::engine::TagEngine,
};

fn main() -> io::Result<()> {
    let (channel, ui_thread) = App::spawn();

    let world = World::generate();
    world.dump();
    let current = Location("woods:entrance".into());

    let machine = StateMachine { world, current };
    let engine = TagEngine::new();

    let engine_thread = Daemon::spawn(machine, engine, channel);

    engine_thread.join().unwrap()?;
    ui_thread.join().unwrap()?;
    Ok(())
}

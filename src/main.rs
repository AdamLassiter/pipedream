use std::io;

use pipedream::{
    interface::app::App,
    resource::{location::Location, world::World},
    engine::{daemon::Daemon, state_machine::StateMachine, tag_engine::TagEngine},
};

fn main() -> io::Result<()> {
    let (channel, ui_thread) = App::spawn();

    let world = World::generate();
    let engine = TagEngine::generate();
    let current = Location("woods:entrance".into());

    let machine = StateMachine {
        world,
        current,
        engine,
    };

    {
        machine.dump();
    }

    let engine_thread = Daemon::spawn(machine, channel);

    engine_thread.join().unwrap()?;
    ui_thread.join().unwrap()?;
    Ok(())
}

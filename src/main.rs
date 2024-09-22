use std::{collections::BTreeMap, io};

use bichannel::BichannelMonitor;
use pipedream::game_coordinator::GameCoordinator;
use pipedream_domain::character::Character;
use pipedream_engine::{
    choice::Card,
    command::UiMode,
    location::Location,
    state::{DynamicStateFn, State},
    state_machine::StateMachine,
};
use pipedream_interface::{log_utils::finish_and_panic_threads, tui::Tui};
use pipedream_prefab::{Generatable, Prefabricated};
use rusqlite::Connection;

fn main() -> io::Result<()> {
    let conn = Connection::open("game.db").expect("Failed to open db");
    Card::initialise(&conn);
    Character::initialise(&conn);
    State::initialise(&conn);

    let (mut monitor, monitor_thread) = BichannelMonitor::spawn();
    let ui_thread = Tui::spawn(&mut monitor);

    let start = (Location::new("woods:entrance"), UiMode::Campaign);
    let states = StateMachine::new(
        conn,
        start,
        BTreeMap::<Location, DynamicStateFn>::generate(),
    );
    let engine_thread = GameCoordinator::spawn(&mut monitor, states);

    finish_and_panic_threads(vec![ui_thread, engine_thread, monitor_thread]);

    Ok(())
}

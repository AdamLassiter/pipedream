#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

use std::{sync::LazyLock, time::Duration};

use rusqlite::Connection;

pub mod cards;
pub mod characters;
pub mod combat_world;
pub mod messages;
pub mod states;

extern crate log;

const COMBAT_ADVANCE_TIME: Duration = Duration::from_secs(1);

pub trait Generatable {
    fn generate() -> Self;
}

pub trait Prefabricated {
    fn initialise(connection: &Connection);
}

pub type Static<T> = LazyLock<T>;

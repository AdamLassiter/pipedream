#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

use std::sync::LazyLock;

use rusqlite::Connection;

pub mod cards;
pub mod characters;
pub mod combat_world;
pub mod states;

extern crate log;

pub trait Generatable {
    fn generate() -> Self;
}

pub trait Prefabricated {
    fn initialise(connection: &Connection);
}

pub type Static<T> = LazyLock<T>;

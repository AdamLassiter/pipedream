#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

pub mod campaign_world;
pub mod cards;
pub mod campaign_state_machine;
pub mod combat_world;
pub mod npcs;
pub mod tag_engine;
pub mod tags;

// Extern log for arbitrary provider
extern crate log;

pub trait Generatable {
    fn generate() -> Self;
}

pub trait Buildable<T> {
    fn build(components: Vec<T>) -> Self;
}

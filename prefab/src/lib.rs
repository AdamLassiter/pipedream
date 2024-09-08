#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]
#![feature(iter_repeat_n)]

pub mod campaign_state_machine;
pub mod campaign_world;
pub mod cards;
pub mod combat_world;
pub mod npcs;

extern crate log;

pub trait Generatable {
    fn generate() -> Self;
}

pub trait Buildable<T> {
    fn build(components: Vec<T>) -> Self;
}

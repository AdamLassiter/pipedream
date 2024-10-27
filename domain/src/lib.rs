#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

pub mod action;
pub mod card;
pub mod character;
pub mod choice;
pub mod class;
pub mod description;
pub mod effect;
pub mod field;
pub mod image;
pub mod location;
pub mod player;
pub mod predicate;
pub mod stats;
pub mod target;

extern crate log;

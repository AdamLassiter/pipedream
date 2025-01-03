#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

pub mod command;
pub mod scene;
pub mod state;
pub mod state_machine;
pub mod tag;

extern crate log;

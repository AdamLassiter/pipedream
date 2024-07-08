#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

pub mod combat;
pub mod core;
pub mod state;

// Extern log for arbitrary provider
extern crate log;

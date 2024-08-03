#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

pub mod game;
pub mod core;
pub mod state;

// Extern log for arbitrary provider
pub extern crate fixed;
pub extern crate log;
pub extern crate rand;
pub extern crate serde;
pub extern crate serde_yml;

#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

// Extern log for arbitrary provider
extern crate log;

pub mod engine;
pub mod interface;
pub mod prefab;
pub mod resource;

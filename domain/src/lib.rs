#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

use std::time::Duration;

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
pub mod message;
pub mod player;
pub mod predicate;
pub mod stats;
pub mod target;

extern crate log;

const AUTO_CPU_TIME: Duration = Duration::from_secs(1);
const AUTO_SKIP_TIME: Duration = Duration::from_secs(0);

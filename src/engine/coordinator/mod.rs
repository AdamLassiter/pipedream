pub mod campaign_coordinator;
pub mod combat_coordinator;

use crate::resource::{commands::UiCommand, transition::Transition};

pub trait Coordinator {
    fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand>;

    fn dump(&self);
}

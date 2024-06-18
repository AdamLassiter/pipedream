pub mod campaign_coordinator;
pub mod combat_coordinator;
pub mod game_coordinator;

use crate::resource::core::{commands::UiCommand, transition::Transition};

pub trait Coordinator {
    fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand>;

    fn dump(&self);
}

pub mod dynamic_state_machine;
pub mod static_state_machine;

use crate::resource::{commands::UiCommand, transition::Transition};

use super::tag_engine::TagEngine;

pub trait StateMachine {
    fn handle_effect(&mut self, engine: &mut TagEngine, side_effect: Transition) -> Vec<UiCommand>;

    fn handle_transition(&mut self, side_effect: Transition);

    fn next_options(&mut self, tag_engine: &TagEngine) -> Vec<UiCommand>;
}

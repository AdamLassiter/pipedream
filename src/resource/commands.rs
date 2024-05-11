use crate::interface::choices::Choices;

use super::{scene::Scene, transition::Transition};

#[derive(Debug)]
pub enum UiCommand {
    Choice(Transition),
    Exit,
}

#[derive(Debug)]
pub enum EngineCommand {
    NewScene(Scene),
    NeedChoice(Choices),
}

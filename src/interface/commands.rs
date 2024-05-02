use crate::{statemachine::transition::Transition, tagengine::action::Action};

use super::{options::Options, scene::Scene};

#[derive(Debug)]
pub enum UiCommand {
    Choice(Transition<Action>),
}

#[derive(Debug)]
pub enum EngineCommand {
    NewScene(Scene),
    NeedChoice(Options<Transition<Action>>),
}

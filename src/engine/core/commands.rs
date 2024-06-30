use super::{choice::Choices, scene::Scene, transition::Transition};

#[derive(Debug)]
pub enum EngineCommand {
    RespondWithChoice(Transition),
    Exit,
}

#[derive(Debug)]
pub enum UiCommand {
    ShowScene(Scene),
    ShowChoices(Choices),
}

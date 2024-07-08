use super::{choice::Choices, scene::Scene, tags::Tags, transition::Transition};

#[derive(Debug)]
pub enum EngineCommand {
    RespondWithChoice(Transition),
    Exit,
}

#[derive(Debug)]
pub enum UiCommand {
    ShowScene(Scene),
    ShowChoices(Choices),
    ShowTags(Tags),
}

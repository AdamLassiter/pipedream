use super::{choice::Choices, scene::Scene, tags::Tags, transition::Transition};

#[derive(Debug, Clone)]
pub enum EngineCommand {
    RespondWithChoice(Transition),
    Exit,
}

#[derive(Debug, Clone)]
pub enum UiCommand {
    ShowScene(Scene),
    ShowChoices(Choices),
    ShowTags(Tags),
    ChangeMode(UiMode),
}

#[derive(Debug, Clone)]
pub enum UiMode {
    Campaign,
    Combat,
}

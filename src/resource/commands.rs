use super::{choice::Choices, scene::Scene, tag::Tags, transition::Transition};

#[derive(Debug)]
pub enum EngineCommand {
    Choice(Transition),
    Exit,
}

#[derive(Debug)]
pub enum UiCommand {
    SceneChange(Scene),
    TagsChange(Tags),
    ChoicesChange(Choices),
}

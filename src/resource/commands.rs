use super::{choice::Choices, scene::Scene, tag::Tags, transition::SideEffect};

#[derive(Debug)]
pub enum EngineCommand {
    Choice(SideEffect),
    Exit,
}

#[derive(Debug)]
pub enum UiCommand {
    SceneChange(Scene),
    TagsChange(Tags),
    ChoicesChange(Choices),
}

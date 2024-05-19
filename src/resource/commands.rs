use super::{choice::Choices, scene::Scene, transition::SideEffect};

#[derive(Debug)]
pub enum EngineCommand {
    Choice(SideEffect),
    Exit,
}

#[derive(Debug)]
pub enum UiCommand {
    SceneChange(Scene),
    ChoicesChange(Choices),
}

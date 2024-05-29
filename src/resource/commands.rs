use super::{choice::Choices, scene::Scene, transition::Transition};

#[derive(Debug)]
pub enum EngineCommand {
    Choice(Transition),
    Exit,
}

#[derive(Debug)]
pub enum UiCommand {
    SceneChange(Scene),
    ChoicesChange(Choices),
}

use super::{options::Options, scene::Scene};

#[derive(Debug)]
pub enum UiCommand {
    Choice(Options),
}

impl UiCommand {
    pub fn get(&self) -> String {
        match self {
            Self::Choice(opts) => opts.options.get(opts.cursor).unwrap().to_string(),
        }
    }
}

#[derive(Debug)]
pub enum EngineCommand {
    NewScene(Scene),
    NeedChoice(Options),
}

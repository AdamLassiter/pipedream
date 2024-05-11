use crate::resource::{commands::EngineCommand, location::Location, state::State, world::World};

#[derive(Debug)]
pub struct StateMachine {
    pub world: World,
    pub current: Location,
}

impl StateMachine {
    pub fn change_state(&mut self, next: Location) -> Vec<EngineCommand> {
        self.current = next;

        let State { scene, options, .. } = self.current_state();
        vec![
            EngineCommand::NewScene(scene.clone()),
            EngineCommand::NeedChoice(options.clone()),
        ]
    }

    pub fn current_state(&self) -> &State {
        self.world.0.get(&self.current).unwrap()
    }
}

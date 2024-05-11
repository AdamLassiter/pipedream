use std::fs::File;

use serde::{Deserialize, Serialize};

use crate::resource::{
    choice::Choice, commands::UiCommand, description::Description, location::Location,
    predicate::Predicate, state::State, transition::Transition, world::World,
};

use super::tag_engine::TagEngine;

#[derive(Serialize, Deserialize)]
pub struct StateMachine {
    pub world: World,
    pub engine: TagEngine,
    pub current: Location,
}

impl StateMachine {
    pub fn handle_transition(&mut self, transition: Transition) -> Vec<UiCommand> {
        self.engine.run_actions(transition.actions);

        self.current = transition.next;

        let State { scene, options, .. } = self.current_state();
        let mut scene = scene.clone();
        let mut options = options.clone();
        let tags = self.engine.tags.clone();

        let test = |predicate: &Option<Predicate>| {
            predicate.is_none()
                || predicate
                    .as_ref()
                    .is_some_and(|pred| self.engine.satisfies(pred))
        };

        scene
            .descriptions
            .retain(|Description { predicate, .. }| test(predicate));
        options.choices.retain(
            |Choice {
                 description: Description { predicate, .. },
                 ..
             }| test(predicate),
        );

        vec![
            UiCommand::SceneChange(scene),
            UiCommand::TagsChange(tags),
            UiCommand::ChoicesChange(options),
        ]
    }

    pub fn current_state(&self) -> &State {
        self.world.0.get(&self.current).unwrap()
    }

    pub fn dump(&self) {
        let buffer = File::create("./state.yaml").unwrap();
        serde_yaml::to_writer(buffer, &self).unwrap();
    }
}

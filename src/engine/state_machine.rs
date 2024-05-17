use serde::{Deserialize, Serialize};

use crate::resource::{
    choice::Choice,
    commands::UiCommand,
    description::Description,
    location::Location,
    predicate::Predicate,
    state::State,
    transition::{SideEffect, TransitionType},
    world::World,
};

use super::tag_engine::TagEngine;

#[derive(Serialize, Deserialize)]
pub struct StateMachine {
    pub world: World,
    pub current: Vec<Location>,
}

impl StateMachine {
    pub fn handle_effect(
        &mut self,
        engine: &mut TagEngine,
        side_effect: SideEffect,
    ) -> Vec<UiCommand> {
        engine.run_actions(&side_effect.actions);

        match side_effect.next {
            TransitionType::Pop => {
                self.current.pop();
            }
            TransitionType::Push(next) => {
                self.current.push(next);
            }
            TransitionType::Swap(next) => {
                self.current.pop();
                self.current.push(next);
            }
            TransitionType::None => {}
        }

        self.next_options(engine)
    }

    fn next_options(&mut self, engine: &TagEngine) -> Vec<UiCommand> {
        let State { scene, options, .. } = self.current_state();
        let mut scene = scene.clone();
        let mut options = options.clone();
        let tags = engine.tags.clone();

        let test = |predicate: &Option<Predicate>| {
            predicate.is_none()
                || predicate
                    .as_ref()
                    .is_some_and(|pred| engine.satisfies(pred))
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
            UiCommand::ChoicesChange(options),
            UiCommand::TagsChange(tags), // debug
        ]
    }

    pub fn current_state(&self) -> &State {
        self.world.0.get(&self.current.last().unwrap()).unwrap()
    }
}

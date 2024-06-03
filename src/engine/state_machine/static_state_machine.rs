use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    engine::tag_engine::TagEngine,
    resource::{
        choice::Choice,
        commands::UiCommand,
        description::Description,
        location::Location,
        predicate::Predicate,
        state::State,
        transition::{Transition, TransitionType},
        world::static_world::StaticWorld,
    },
};

use super::StateMachine;

#[derive(Serialize, Deserialize)]
pub struct StaticStateMachine<W: StaticWorld> {
    pub world: W,
    pub current: Vec<Location>,
}

impl<W: StaticWorld> StateMachine for StaticStateMachine<W> {
    fn handle_effect(
        &mut self,
        tag_engine: &mut TagEngine,
        side_effect: Transition,
    ) -> Vec<UiCommand> {
        tag_engine.handle_actions(&side_effect.actions);
        self.handle_transition(side_effect);

        self.next_options(tag_engine)
    }

    fn handle_transition(&mut self, side_effect: Transition) {
        debug!(target:"Event/Transition", "{:?}", side_effect.next);

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
        };

        debug!(target:"State/Location", "{:?}", self.current);
    }

    fn next_options(&mut self, tag_engine: &TagEngine) -> Vec<UiCommand> {
        let State { scene, options, .. } = self.current_state();
        let mut scene = scene.clone();
        let mut options = options.clone();

        let test = |predicate: &Option<Predicate>| {
            predicate.is_none()
                || predicate
                    .as_ref()
                    .is_some_and(|pred| tag_engine.satisfies(pred))
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

        vec![UiCommand::ShowScene(scene), UiCommand::ShowChoices(options)]
    }
}

impl<W: StaticWorld> StaticStateMachine<W> {
    fn current_state(&self) -> State {
        self.world.get_state(self.current.last().unwrap()).clone()
    }
}

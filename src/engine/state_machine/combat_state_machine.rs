use log::debug;

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
        world::combat_world::{CombatWorld, DynamicWorld},
    },
};

pub struct CombatStateMachine {
    pub world: CombatWorld,
    pub current: Vec<Location>,
}

impl CombatStateMachine {
    pub fn handle_effect(&mut self, engine: &mut TagEngine, side_effect: Transition) -> Vec<UiCommand> {
        engine.handle_actions(&side_effect.actions);
        self.handle_transition(side_effect);

        self.next_options(engine)
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
        let State { scene, options, .. } = self.current_state(tag_engine);
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

    fn current_state(&self, tag_engine: &TagEngine) -> State {
        let state_fn = self.world.get_state(self.current.last().unwrap());

        state_fn.apply(tag_engine)
    }
}

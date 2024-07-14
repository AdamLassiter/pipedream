use log::debug;
use serde::Serialize;

use crate::core::{
    choice::{Choice, ChoiceType},
    commands::UiCommand,
    description::Description,
    location::Location,
    predicate::Predicate,
    state::State,
    transition::{Transition, TransitionType},
};

use super::{combat_world::CombatWorld, tag_engine::TagEngine};

#[derive(Serialize)]
pub struct CombatStateMachine {
    #[serde(skip_serializing)]
    pub combat_world: CombatWorld,
    #[serde(skip_serializing)]
    pub exporter: fn(&Self) -> Transition,
    pub tag_engine: TagEngine,
    pub current: Vec<Location>,
}

impl CombatStateMachine {
    pub fn new(
        combat_world: CombatWorld,
        tag_engine: TagEngine,
        start: Location,
        exporter: fn(&Self) -> Transition,
    ) -> Self {
        Self {
            combat_world,
            exporter,
            tag_engine,
            current: vec![start],
        }
    }

    pub fn handle_effect(&mut self, side_effect: Transition) -> Result<Vec<UiCommand>, Transition> {
        self.tag_engine.handle_actions(&side_effect.actions);
        self.handle_transition(side_effect);

        if self.current.is_empty() {
            Err((self.exporter)(self))
        } else {
            Ok(self.next_options())
        }
    }

    fn handle_transition(&mut self, side_effect: Transition) {
        debug!(target:"Event/Transition", "{:?}", side_effect.next);

        match side_effect.next {
            TransitionType::Leave => {
                self.current.pop();
            }
            TransitionType::Enter(next) => {
                self.current.push(next);
            }
            TransitionType::Goto(next) => {
                self.current.pop();
                self.current.push(next);
            }
            TransitionType::Combat(_init) => {
                panic!("Can't enter combat while already in combat");
            }
            TransitionType::None => {}
        };

        debug!(target:"State/Location", "{:?}", self.current);
    }

    pub fn next_options(&mut self) -> Vec<UiCommand> {
        let State { scene, options, .. } = self.current_state();
        let mut scene = scene.clone();
        let mut options = options.clone();
        let tags = self.tag_engine.tags.clone();

        let test = |predicate: &Option<Predicate>| {
            predicate.is_none()
                || predicate
                    .as_ref()
                    .is_some_and(|pred| self.tag_engine.satisfies(pred))
        };

        scene
            .descriptions
            .retain(|Description { predicate, .. }| test(predicate));
        if let ChoiceType::Manual(ref mut choices) = options.choices {
            choices.retain(|Choice { predicate, .. }| test(predicate));
        }

        debug!(target:"Event/Scene", "{:?}", &scene);
        debug!(target:"Event/Choices", "{:?}", &options);
        debug!(target:"Event/Tags", "{:?}", &tags);
        vec![
            UiCommand::ShowScene(scene),
            UiCommand::ShowChoices(options),
            UiCommand::ShowTags(tags),
        ]
    }

    fn current_state(&self) -> State {
        let state_fn = self.combat_world.get_state(
            self.current
                .last()
                .expect("Location stack empty, cannot find current state"),
        );

        state_fn.apply(self)
    }
}

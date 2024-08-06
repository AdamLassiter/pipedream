use std::collections::BTreeMap;

use log::debug;
use rusqlite::Connection;

use crate::core::{
    choice::{Choice, Choices},
    command::UiCommand,
    description::Description,
    effect::Effect,
    location::Location,
    predicate::Predicate,
    state::State,
};

use super::{action::Action, effect::Transition, state::DynamicStateFn};

pub struct StateMachine {
    pub connection: Connection,
    pub locations: Vec<Location>,
    pub dynamic_states: BTreeMap<Location, DynamicStateFn>,
}

impl StateMachine {
    pub fn new(
        connection: Connection,
        start: Location,
        dynamic_states: BTreeMap<Location, DynamicStateFn>,
    ) -> Self {
        Self {
            connection,
            locations: vec![start],
            dynamic_states,
        }
    }

    pub fn handle_effect(&mut self, effect: Effect) -> Vec<UiCommand> {
        self.handle_actions(effect.actions);
        self.handle_transition(effect.transition);
        self.next_options()
    }

    fn handle_actions(&mut self, actions: Vec<Action>) {
        debug!(target:"Engine/StateMachine/HandleActions", "{:?}", actions);

        actions.iter().for_each(|action| action.run());
    }

    fn handle_transition(&mut self, transition: Transition) {
        debug!(target:"Engine/StateMachine/HandleTransition", "{:?}", transition);

        match transition {
            Transition::Leave => {
                self.locations.pop();
            }
            Transition::Enter(next) => {
                self.locations.push(next);
            }
            Transition::Goto(next) => {
                self.locations.pop();
                self.locations.push(next);
            }
            Transition::None => {}
        };

        debug!(target:"Engine/StateMachine/LocationState", "{:?}", self.locations);
    }

    pub fn next_options(&mut self) -> Vec<UiCommand> {
        let location = self.location();
        let State { scene, choices, .. } = self.state(location);
        let mut scene = scene.clone();
        let mut choices = choices.clone();

        let test = |predicate: &Option<Predicate>| {
            predicate
                .as_ref()
                .map(|predicate| predicate.test())
                .unwrap_or(true)
        };

        scene
            .0
            .retain(|Description { predicate, .. }| test(predicate));

        if let Choices::Manual(ref mut choices) = choices {
            choices.retain(|Choice { predicate, .. }| test(predicate));
        }

        debug!(target:"Engine/StateMachine/ShowScene", "{:?}", &scene);
        debug!(target:"Engine/StateMachine/ShowChoices", "{:?}", &choices);
        vec![
            UiCommand::ShowScene(scene),
            UiCommand::ShowChoices(choices),
            UiCommand::ChangeMode(location.ui_mode.clone()),
        ]
    }

    fn location(&self) -> &Location {
        self.locations.last().expect("Location stack empty")
    }

    fn state(&self, location: &Location) -> State {
        if let Some(state_fn) = self.dynamic_states.get(location) {
            state_fn.apply(self)
        } else {
            State::query_by_location(&self.connection, &location.location)
                .expect("Failed to query db")
                .pop()
                .unwrap_or_else(|| panic!(
                    "Failed to query states by location {}",
                    location.location
                ))
        }
    }
}

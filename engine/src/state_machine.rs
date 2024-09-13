use std::collections::BTreeMap;

use log::debug;
use rusqlite::Connection;

use crate::{
    choice::{Card, Choice, Choices},
    command::UiCommand,
    description::Description,
    effect::Effect,
    location::Location,
    predicate::Predicate,
    state::State,
};

use super::{action::Action, effect::Transition, state::DynamicStateFn};

pub struct StateMachine {
    pub conn: Connection,
    pub location_stack: Vec<Location>,
    pub dynamic_states: BTreeMap<Location, DynamicStateFn>,
}

impl StateMachine {
    pub fn new(
        connection: Connection,
        start: Location,
        dynamic_states: BTreeMap<Location, DynamicStateFn>,
    ) -> Self {
        Self {
            conn: connection,
            location_stack: vec![start],
            dynamic_states,
        }
    }

    pub fn handle_effect(&mut self, effect: Effect) -> Vec<UiCommand> {
        effect
            .actions
            .into_iter()
            .for_each(|action| self.handle_action(action));
        self.handle_transition(effect.transition);
        self.next_options()
    }

    fn handle_action(&mut self, action: Action) {
        debug!(target:"Engine/StateMachine/HandleActions", "{:?}", action);

        action.run(&mut self.conn).expect("Failed to run action");
    }

    fn handle_transition(&mut self, transition: Transition) {
        debug!(target:"Engine/StateMachine/HandleTransition", "{:?}", transition);

        match transition {
            Transition::Leave => {
                self.location_stack.pop();
            }
            Transition::Enter(next) => {
                self.location_stack.push(next);
            }
            Transition::Goto(next) => {
                self.location_stack.pop();
                self.location_stack.push(next);
            }
            Transition::None => {}
        };

        debug!(target:"Engine/StateMachine/LocationState", "{:?}", self.location_stack);
    }

    pub fn next_options(&mut self) -> Vec<UiCommand> {
        let location = self.location();
        let State { scene, choices, .. } = self.state(location);
        let mut scene = scene.clone();
        let mut choices = choices.clone();

        let test = |conn: &Connection, predicate: &Option<Predicate>| {
            predicate
                .as_ref()
                .map(|predicate| {
                    predicate
                        .clone()
                        .test(conn)
                        .unwrap_or_else(|_| panic!("Failed to execute predicate {:?}", predicate))
                })
                .unwrap_or(true)
        };

        scene
            .descriptions
            .retain(|Description { predicate, .. }| test(&self.conn, predicate));

        if let Choices::Manual(ref mut choices) = choices {
            choices.retain(
                |Choice {
                     card: Card { predicate, .. },
                     ..
                 }| test(&self.conn, predicate),
            );
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
        self.location_stack.last().expect("Location stack empty")
    }

    fn state(&self, location: &Location) -> State {
        if let Some(state_fn) = self.dynamic_states.get(location) {
            state_fn.apply(self)
        } else {
            let (_id, state) = State::query_by_location(&self.conn, &location.location)
                .ok()
                .and_then(|mut res| res.pop())
                .unwrap_or_else(|| {
                    panic!("Failed to query states by location {}", location.location)
                });
            state
        }
    }
}

use std::collections::BTreeMap;

use log::debug;
use rusqlite::Connection;

use crate::{
    choice::{Card, Choice, Choices},
    command::{UiCommand, UiMode},
    description::Description,
    effect::Effect,
    location::Location,
    predicate::Predicate,
    state::{State, StateDao},
};

use super::{action::Action, effect::Transition, state::DynamicStateFn};

pub struct StateMachine {
    pub conn: Connection,
    pub location_stack: Vec<(Location, UiMode)>,
    pub dynamic_states: BTreeMap<Location, DynamicStateFn>,
}

impl StateMachine {
    pub fn new(
        connection: Connection,
        start: (Location, UiMode),
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
                self.location_stack.push((next, UiMode::Campaign));
            }
            Transition::Fight(next) => {
                self.location_stack.push((next, UiMode::Combat));
            }
            Transition::Goto(next) => {
                let (_, ui_mode) = self.location_stack.pop().expect("Location stack empty");
                self.location_stack.push((next, ui_mode));
            }
            Transition::None => {}
        };

        debug!(target:"Engine/StateMachine/LocationState", "{:?}", self.location_stack);
    }

    pub fn next_options(&mut self) -> Vec<UiCommand> {
        let (location, ui_mode) = self.location();
        let State { scene, choices, .. } = self.state(location);
        let mut scene = scene.clone();
        let mut choices = choices.clone();

        let test = |conn: &Connection, predicate: &Option<Predicate>| {
            predicate
                .as_ref()
                .map(|predicate| {
                    predicate.clone().test(conn).unwrap_or_else(|e| {
                        panic!("Failed to execute predicate {:?}: {}", predicate, e)
                    })
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
            UiCommand::ChangeMode(ui_mode.clone()),
        ]
    }

    fn location(&self) -> &(Location, UiMode) {
        self.location_stack.last().expect("Location stack empty")
    }

    fn state(&self, location: &Location) -> State {
        if let Some(state_fn) = self.dynamic_states.get(location) {
            state_fn.apply(self)
        } else {
            let state = StateDao::select_location(&self.conn, location)
                .unwrap_or_else(|e| panic!("Failed to find Location for {:?}: {}", location, e))
                .pop()
                .unwrap_or_else(|| panic!("No Location found for {:?}", location));
            state.into()
        }
    }
}

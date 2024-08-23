use std::collections::BTreeMap;

use log::debug;
use rusqlite::Connection;

use crate::{
    core::{
        choice::{Choice, Choices},
        command::UiCommand,
        description::Description,
        effect::Effect,
        location::Location,
        predicate::Predicate,
        state::State,
    },
    domain::{
        character::Character,
        encounter::{Player, PlayerCharacter},
        stats::StatChange,
    },
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
            .descriptions
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
            let (_id, state) = State::query_by_location(&self.connection, &location.location)
                .ok()
                .and_then(|mut res| res.pop())
                .unwrap_or_else(|| {
                    panic!("Failed to query states by location {}", location.location)
                });
            state
        }
    }

    pub fn get_character(&self, player: &Player) -> Character {
        let (_id, PlayerCharacter { character, .. }) =
            PlayerCharacter::query_by_player(&self.connection, &player)
                .ok()
                .and_then(|mut res| res.pop())
                .unwrap_or_else(|| panic!("Failed to find EncounterCharacter for {:?}", player));
        character
    }

    pub fn update_character<T>(&self, player: &Player, update_fn: T) where T: FnOnce(&mut Character) {
        let (id, PlayerCharacter { mut character, .. }) =
            PlayerCharacter::query_by_player(&self.connection, &player)
                .ok()
                .and_then(|mut res| res.pop())
                .unwrap_or_else(|| panic!("Failed to find EncounterCharacter for {:?}", player));
        update_fn(&mut character);
        character.update(&self.connection, id)
            .ok()
            .unwrap_or_else(|| panic!("Failed to save EncounterCharacter for {:?}", player));
    }

    pub fn get_source_stat_changes(&self, source: &Player) -> Vec<StatChange> {
        let (_id, stat_changes) = StatChange::query_by_source(&self.connection, &source)
            .ok()
            .unwrap_or_else(|| panic!("Failed to find StatChanges for {:?}", source))
            .into_iter()
            .unzip::<i64, StatChange, Vec<_>, Vec<_>>();
        stat_changes
    }

    pub fn get_target_stat_changes(&self, target: &Player) -> Vec<StatChange> {
        let (_id, stat_changes) = StatChange::query_by_target(&self.connection, &target)
            .ok()
            .unwrap_or_else(|| panic!("Failed to find StatChanges for {:?}", target))
            .into_iter()
            .unzip::<i64, StatChange, Vec<_>, Vec<_>>();
        stat_changes
    }
}

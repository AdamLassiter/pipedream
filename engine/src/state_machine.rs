use std::collections::BTreeMap;

use log::debug;
use rusqlite::Connection;

use crate::{
    command::{UiCommand, UiMode},
    state::{DynamicStateFn, State, StateDao},
};
use pipedream_domain::{
    action::Action,
    character::Character,
    choice::{Choice, Choices},
    description::Description,
    effect::{Effect, Transition},
    image::Image,
    location::{Location, LocationStack},
    player::{Player, PlayerCharacter},
    predicate::Predicate,
    stats::Stats,
};

pub struct StateMachine {
    pub conn: Connection,
    pub location_stack: Vec<(Location, UiMode)>,
    pub dynamic_states: BTreeMap<Location, DynamicStateFn>,
}

impl StateMachine {
    pub fn new(
        conn: Connection,
        start: (Location, UiMode),
        dynamic_states: BTreeMap<Location, DynamicStateFn>,
    ) -> Self {
        Self {
            conn,
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
        let locations = LocationStack(
            self.location_stack
                .iter()
                .map(|(location, _)| location.clone())
                .collect::<Vec<_>>(),
        );
        let State { scene, choices, .. } = self.state(location);
        let mut scene = scene.clone();
        let mut choices = choices.clone();
        let (human_image, human_stats) = self.stats(&Player::Human);
        let (cpu_image, cpu_stats) = self.stats(&Player::Cpu);

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

        if let Choices::Manual(ref mut choices) = choices
            && *ui_mode == UiMode::Campaign
        {
            choices.retain(|Choice { predicate, .. }| test(&self.conn, predicate));
        }

        debug!(target:"Engine/StateMachine/Location", "{:?}", location);
        debug!(target:"Engine/StateMachine/UiMode", "{:?}", ui_mode);
        debug!(target:"Engine/StateMachine/ShowScene", "{:?}", &scene);
        debug!(target:"Engine/StateMachine/ShowChoices", "{:?}", &choices);
        vec![
            UiCommand::ChangeMode(ui_mode.clone()),
            UiCommand::ShowPortrait(Player::Human, human_image),
            UiCommand::ShowPortrait(Player::Cpu, cpu_image),
            UiCommand::ShowStats(Player::Human, human_stats),
            UiCommand::ShowStats(Player::Cpu, cpu_stats),
            UiCommand::ShowLocation(locations),
            UiCommand::ShowScene(scene),
            UiCommand::ShowChoices(choices),
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

    fn stats(&self, player: &Player) -> (Option<Image>, Option<Stats>) {
        if let Some((_id, Character { image, stats, .. })) =
            PlayerCharacter::find_player_character(&self.conn, player)
        {
            (Some(image), Some(stats))
        } else {
            (None, None)
        }
    }
}

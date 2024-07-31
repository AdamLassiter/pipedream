use log::debug;
use serde::Serialize;

use crate::core::{
    choice::{Choice, ChoiceType},
    command::{UiCommand, UiMode},
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
    pub current: Option<Location>,
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
            current: Some(start),
        }
    }

    pub fn handle_effect(
        &mut self,
        side_effect: Transition,
    ) -> (Vec<UiCommand>, Option<Transition>) {
        self.tag_engine.handle_actions(&side_effect.actions);
        let handle_transition = self.handle_transition(side_effect);
        debug!(target:"Engine/CombatStateMachine/HandleTransition", "{:?}", handle_transition);

        let handle_combat = handle_transition
            .map(|exit| (exit, Some((self.exporter)(self))))
            .unwrap_or_else(|| (self.next_options(), None));
        debug!(target:"Engine/CombatStateMachine/HandleCombat", "{:?}", handle_combat);

        handle_combat
    }

    fn handle_transition(&mut self, side_effect: Transition) -> Option<Vec<UiCommand>> {
        debug!(target:"Engine/CombatStateMachine/HandleTransition", "{:?}", side_effect.next);

        match side_effect.next {
            TransitionType::Leave => {
                self.current = None;
                let exit_combat_cmds = vec![UiCommand::ChangeMode(UiMode::Campaign)];
                return Some(exit_combat_cmds);
            }
            TransitionType::Enter(next) => {
                self.current = Some(next);
            }
            TransitionType::Goto(next) => {
                self.current = Some(next);
            }
            TransitionType::Combat(_init) => {
                panic!("Can't enter combat while already in combat");
            }
            TransitionType::None => {}
        };

        debug!(target:"Engine/CombatStateMachine/LocationState", "{:?}", self.current);
        None
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

        debug!(target:"Engine/CombatStateMachine/ShowScene", "{:?}", &scene);
        debug!(target:"Engine/CombatStateMachine/ShowChoices", "{:?}", &options);
        debug!(target:"Engine/CombatStateMachine/ShowTags", "{:?}", &tags);
        vec![
            UiCommand::ShowScene(scene),
            UiCommand::ShowChoices(options),
            UiCommand::ShowTags(tags),
        ]
    }

    fn current_state(&self) -> State {
        let state_fn = self.combat_world.get_state(
            self.current
                .as_ref()
                .expect("Location stack empty, cannot find current state"),
        );

        state_fn.apply(self)
    }
}

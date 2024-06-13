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
        world::combat_world::CombatWorld,
    },
};

pub struct CombatStateMachine {
    pub combat_world: CombatWorld,
    pub current: Vec<Location>,
}

impl CombatStateMachine {
    pub fn init(&mut self, tag_engine: &mut TagEngine) -> Vec<UiCommand> {
        self.handle_effect(
            tag_engine,
            Transition {
                next: crate::resource::transition::TransitionType::Enter(Location(
                    "combat:init".into(),
                )),
                actions: vec![],
            },
        )
    }

    pub fn handle_effect(
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
                panic!(); // Can't combat while in combat
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
        let state_fn = self.combat_world.get_state(self.current.last().unwrap());

        state_fn.apply(tag_engine)
    }
}

impl Default for CombatStateMachine {
    fn default() -> Self {
        Self {
            combat_world: CombatWorld::generate(),
            current: vec![],
        }
    }
}

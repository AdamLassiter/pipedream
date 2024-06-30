use log::debug;
use serde::Serialize;

use crate::engine::core::{
    choice::{Choice, ChoiceType},
    commands::UiCommand,
    description::Description,
    location::Location,
    predicate::Predicate,
    state::State,
    tag::Static,
    transition::{Transition, TransitionType},
};

use super::{
    campaign_state_machine::CampaignStateMachine, combat_world::CombatWorld, tag_engine::TagEngine,
};

pub static COMBAT_INIT: Static<Location> = Static::new(|| Location("combat:init".to_string()));
pub static PLAYER_DRAW: Static<Location> = Static::new(|| Location("player:draw".to_string()));
pub static PLAYER_PLAY: Static<Location> = Static::new(|| Location("player:play".to_string()));
pub static PLAYER_RESOLVE_PLAY: Static<Location> =
    Static::new(|| Location("player:play:resolve".to_string()));

#[derive(Serialize)]
pub struct CombatStateMachine {
    #[serde(skip_serializing)]
    pub combat_world: CombatWorld,
    pub tag_engine: TagEngine,
    pub current: Vec<Location>,
}

impl CombatStateMachine {
    pub fn new(combat_world: CombatWorld, tag_engine: TagEngine, start: Location) -> Self {
        Self {
            combat_world,
            tag_engine,
            current: vec![start],
        }
    }

    pub fn from_campaign(campaign_machine: &CampaignStateMachine) -> Self {
        Self::new(
            CombatWorld::generate(),
            TagEngine::from_campaign(&campaign_machine.tag_engine),
            COMBAT_INIT.clone(),
        )
    }

    pub fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand> {
        self.tag_engine.handle_actions(&side_effect.actions);
        self.handle_transition(side_effect);

        self.next_options()
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

    pub fn next_options(&mut self) -> Vec<UiCommand> {
        let State { scene, options, .. } = self.current_state();
        let mut scene = scene.clone();
        let mut options = options.clone();

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
            choices.retain(
                |Choice {
                     description: Description { predicate, .. },
                     ..
                 }| test(predicate),
            );
        }

        debug!(target:"Event/Render", "{:?}", &scene);
        debug!(target:"Event/Query", "{:?}", &options);
        vec![UiCommand::ShowScene(scene), UiCommand::ShowChoices(options)]
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

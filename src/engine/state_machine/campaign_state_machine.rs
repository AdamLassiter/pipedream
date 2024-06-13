use log::debug;
use serde::{Deserialize, Serialize};

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
        world::campaign_world::CampaignWorld,
    },
};

use super::combat_state_machine::CombatStateMachine;

#[derive(Serialize, Deserialize)]
pub struct CampaignStateMachine {
    pub campaign_world: CampaignWorld,
    #[serde(default = "none", skip_serializing, skip_deserializing)]
    pub combat_state_machine: Option<CombatStateMachine>,
    pub current: Vec<Location>,
}

fn none() -> Option<CombatStateMachine> {
    None
}

impl CampaignStateMachine {
    pub fn new(campaign_world: CampaignWorld) -> Self {
        Self {
            campaign_world,
            combat_state_machine: None,
            current: vec![],
        }
    }

    pub fn handle_effect(
        &mut self,
        tag_engine: &mut TagEngine,
        side_effect: Transition,
    ) -> Vec<UiCommand> {
        if let Some(combat_state_machine) = self.combat_state_machine.as_mut() {
            return combat_state_machine.handle_effect(tag_engine, side_effect);
        }

        tag_engine.handle_actions(&side_effect.actions);
        self.handle_transition(side_effect, tag_engine)
            .unwrap_or_else(|| self.next_options(tag_engine))
    }

    fn handle_transition(
        &mut self,
        side_effect: Transition,
        tag_engine: &mut TagEngine,
    ) -> Option<Vec<UiCommand>> {
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
            TransitionType::Combat(init_actions) => {
                tag_engine.handle_actions(&init_actions);
                let combat_init = self
                    .combat_state_machine
                    .get_or_insert(CombatStateMachine::default())
                    .init(tag_engine);
                return Some(combat_init);
            }
            TransitionType::None => {}
        };

        debug!(target:"State/Location", "{:?}", self.current);
        None
    }

    fn next_options(&mut self, tag_engine: &TagEngine) -> Vec<UiCommand> {
        let State { scene, options, .. } = self.current_state();
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

    fn current_state(&self) -> State {
        self.campaign_world
            .get_state(self.current.last().unwrap())
            .clone()
    }
}

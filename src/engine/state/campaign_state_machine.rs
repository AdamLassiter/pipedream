use log::debug;
use serde::{Deserialize, Serialize};

use crate::engine::core::{
    choice::{Choice, ChoiceType},
    commands::UiCommand,
    description::Description,
    location::Location,
    predicate::Predicate,
    state::State,
    transition::{Transition, TransitionType},
};

use super::{
    campaign_world::CampaignWorld, combat_state_machine::CombatStateMachine, tag_engine::TagEngine,
};

#[derive(Serialize, Deserialize)]
pub struct CampaignStateMachine {
    pub campaign_world: CampaignWorld,
    pub tag_engine: TagEngine,
    #[serde(default = "none", skip_serializing, skip_deserializing)]
    pub combat_state_machine: Option<CombatStateMachine>,
    pub current: Vec<Location>,
}

fn none() -> Option<CombatStateMachine> {
    None
}

impl CampaignStateMachine {
    pub fn new(campaign_world: CampaignWorld, tag_engine: TagEngine, start: Location) -> Self {
        Self {
            campaign_world,
            tag_engine,
            combat_state_machine: None,
            current: vec![start],
        }
    }

    pub fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand> {
        if let Some(combat_state_machine) = self.combat_state_machine.as_mut() {
            return combat_state_machine.handle_effect(side_effect);
        }

        self.tag_engine.handle_actions(&side_effect.actions);
        self.handle_transition(side_effect)
            .unwrap_or_else(|| self.next_options())
    }

    fn handle_transition(&mut self, side_effect: Transition) -> Option<Vec<UiCommand>> {
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
                self.tag_engine.handle_actions(&init_actions);
                let combat_init = self
                    .combat_state_machine
                    .insert(CombatStateMachine::from_campaign(self));
                return Some(combat_init.next_options());
            }
            TransitionType::None => {}
        };

        debug!(target:"State/Location", "{:?}", self.current);
        None
    }

    fn next_options(&mut self) -> Vec<UiCommand> {
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
        self.campaign_world
            .get_state(
                self.current
                    .last()
                    .expect("Location stack empty, cannot find current state"),
            )
            .clone()
    }
}

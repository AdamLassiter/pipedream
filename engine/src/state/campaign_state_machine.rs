use log::debug;
use serde::{Deserialize, Serialize};

use crate::core::{
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
    #[serde(skip_serializing, skip_deserializing)]
    pub exporter: Option<fn(&Self) -> CombatStateMachine>,
    pub tag_engine: TagEngine,
    #[serde(default = "none", skip_serializing, skip_deserializing)]
    pub combat_state_machine: Option<CombatStateMachine>,
    pub current: Vec<Location>,
}

fn none() -> Option<CombatStateMachine> {
    None
}

impl CampaignStateMachine {
    pub fn new(
        campaign_world: CampaignWorld,
        tag_engine: TagEngine,
        start: Location,
        exporter: fn(&Self) -> CombatStateMachine,
    ) -> Self {
        Self {
            campaign_world,
            exporter: Some(exporter),
            tag_engine,
            combat_state_machine: None,
            current: vec![start],
        }
    }

    pub fn handle_effect(&mut self, side_effect: Transition) -> Vec<UiCommand> {
        // If in combat, handle effect in combat
        // This might bubble-up something for the campaign to  handle
        let handle_combat = self
            .combat_state_machine
            .as_mut()
            .map(|csm| csm.handle_effect(side_effect.clone()));
        debug!(target:"Handle/Combat", "{:?}", handle_combat);

        // If combat ended, pop the combat-state-machine
        // Otherwise, prepare to handle effect in campaign
        let handle_end_combat = handle_combat
            .map(|csm| {
                csm.inspect_err(|_ended_combat| {
                    self.combat_state_machine.take();
                })
            })
            .unwrap_or_else(|| Err(side_effect));
        debug!(target:"Handle/EndCombat", "{:?}", handle_end_combat);

        // If not in combat, handle effect in campaign
        let handle_campaign = handle_end_combat.unwrap_or_else(|side_effect| {
            self.tag_engine.handle_actions(&side_effect.actions);
            self.handle_transition(side_effect)
                .unwrap_or_else(|| self.next_options())
        });
        debug!(target:"Handle/Campaign", "{:?}", handle_campaign);

        handle_campaign
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
                let combat_init = self.combat_state_machine.insert(self
                    .exporter
                    .expect("Failed to get Campaign -> Combat exporter")(
                    self
                ));
                return Some(combat_init.next_options());
            }
            TransitionType::None => {}
        };

        debug!(target:"State/Location", "{:?}", self.current);
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
            choices.retain(
                |Choice {
                     description: Description { predicate, .. },
                     ..
                 }| test(predicate),
            );
        }

        debug!(target:"Show/Scene", "{:?}", &scene);
        debug!(target:"Show/Choices", "{:?}", &options);
        debug!(target:"Show/Tags", "{:?}", &tags);
        vec![
            UiCommand::ShowScene(scene),
            UiCommand::ShowChoices(options),
            UiCommand::ShowTags(tags),
        ]
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

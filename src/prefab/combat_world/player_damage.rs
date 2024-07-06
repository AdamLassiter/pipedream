use log::debug;

use crate::{
    engine::{
        core::{
            action::Action,
            scene::Scene,
            state::State,
            tag::{Tag, TagKey, TagValue, Tags, FI64},
            transition::{Transition, TransitionType},
        },
        state::{combat_state_machine::*, combat_world::*, tag_engine::ANY_RESOURCE},
    },
    prefab::combat_world::{
        ANY_ATTRIBUTE_RESIST, ANY_RESOURCE_DAMAGE, MY_ATTRIBUTE_ASSIST, PLAYER_DAMAGE, PLAYER_PLAY,
    },
};

impl CombatWorld {
    fn calculate_damage(assist_stat: FI64, resist_stat: FI64, damage_val: FI64) -> FI64 {
        (assist_stat / resist_stat).sqrt() * damage_val
    }

    pub fn player_damamge_phase(machine: &CombatStateMachine) -> State {
        let any_resc_damage_slice = machine.tag_engine.find(&ANY_RESOURCE_DAMAGE);
        debug!(target:"Combat/Damage", "{:?}", any_resc_damage_slice);
        let my_attr_assist_slice: Tags = machine.tag_engine.find(&MY_ATTRIBUTE_ASSIST).into();
        debug!(target:"Combat/Assist", "{:?}", my_attr_assist_slice);
        let any_attr_resist_slice: Tags = machine.tag_engine.find(&ANY_ATTRIBUTE_RESIST).into();
        debug!(target:"Combat/Resist", "{:?}", any_attr_resist_slice);

        let resolved_damages = any_resc_damage_slice
            .into_iter()
            .map(|Tag { key, value }| {
                let target = key.leading_key();
                let dmg_type = key.trailing_key();
                let assist_stat = match my_attr_assist_slice.get(&TagKey::from(
                    format!("{}:{}", MY_ATTRIBUTE_ASSIST.0, dmg_type).as_ref(),
                )) {
                    Some(TagValue::Number(num)) => *num,
                    _ => 0.into(),
                };
                let resist_stat = match my_attr_assist_slice.get(&TagKey::from(
                    format!(
                        "{}:{}:{}",
                        target,
                        ANY_ATTRIBUTE_RESIST.leading_subpath().0,
                        dmg_type
                    )
                    .as_ref(),
                )) {
                    Some(TagValue::Number(num)) => *num,
                    _ => 0.into(),
                };
                let damage_val = match value {
                    TagValue::Number(num) => num,
                    _ => 0.into(),
                };
                let calculated_dmg = Self::calculate_damage(assist_stat, resist_stat, damage_val);
                Action::Subtract(
                    format!(
                        "{}:{}:{}/{}",
                        target,
                        ANY_RESOURCE.leading_subpath().0,
                        dmg_type,
                        calculated_dmg,
                    )
                    .into(),
                )
            })
            .collect::<Vec<_>>();

        State {
            location: PLAYER_DAMAGE.clone(),
            scene: Scene {
                descriptions: vec![],
            },
            options: Transition {
                next: TransitionType::Goto(PLAYER_PLAY.clone()),
                actions: resolved_damages,
            }
            .into(),
        }
    }
}

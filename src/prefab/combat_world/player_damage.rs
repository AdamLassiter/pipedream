use log::debug;

use crate::{
    engine::{
        core::{
            action::Action,
            scene::Scene,
            state::State,
            tag::{Tag, TagKey, TagValue, FI64},
            transition::{Transition, TransitionType},
        },
        state::{combat_state_machine::CombatStateMachine, combat_world::CombatWorld},
    },
    prefab::{
        combat_world::{COMBAT_VICTORY, PLAYER_DAMAGE, PLAYER_PLAY},
        tag_engine::Ent,
        tags::Tgt,
    },
};

impl CombatWorld {
    fn calculate_damage(assist_stat: FI64, resist_stat: FI64, damage_val: FI64) -> FI64 {
        (assist_stat / resist_stat).sqrt() * damage_val
    }

    pub fn player_damamge_phase(machine: &CombatStateMachine) -> State {
        let any_resc_damage_slice = machine.tag_engine.find(&Tgt::Any.ent(Ent::Damage));
        debug!(target:"Combat/Damage", "{:?}", any_resc_damage_slice);

        let resolved_damages = any_resc_damage_slice
            .into_iter()
            .map(|Tag { key, value }| {
                let target = key.leading_key();
                let dmg_type = key.trailing_key();

                let assist_stat = match machine
                    .tag_engine
                    .find(&TagKey::from(
                        format!("{}:{}:{}", Tgt::Me, Ent::AttributeAssist, dmg_type).as_ref(),
                    ))
                    .first()
                {
                    Some(Tag {
                        value: TagValue::Number(num),
                        ..
                    }) => *num,
                    _ => 1.into(),
                };
                let resist_stat = match machine
                    .tag_engine
                    .find(&TagKey::from(
                        format!("{}:{}:{}", target, Ent::AttributeResist, dmg_type).as_ref(),
                    ))
                    .first()
                {
                    Some(Tag {
                        value: TagValue::Number(num),
                        ..
                    }) => *num,
                    _ => 1.into(),
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
                        Ent::Resource,
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
                next: TransitionType::Goto(COMBAT_VICTORY.clone()),
                actions: resolved_damages,
            }
            .into(),
        }
    }
}

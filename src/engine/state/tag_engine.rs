use log::debug;
use serde::{Deserialize, Serialize};

use crate::engine::{
    core::{
        action::Action,
        predicate::Predicate,
        tag::{Static, Tag, TagKey, TagValue, Tags, FI64, ME_REF, YOU_REF},
    },
    state::combat_world::{ENEMY, PLAYER},
};

pub static ANY: Static<String> = Static::new(|| "$any".into());
pub static ANY_SUBSTITUTIONS: Static<Vec<String>> =
    Static::new(|| vec![PLAYER.0.clone(), ENEMY.0.clone()]);
pub static ANY_NAME: Static<TagKey> = Static::new(|| "$any:name".into());
pub static ANY_ATTRIBUTE: Static<TagKey> = Static::new(|| "$any:attribute".into());
pub static ANY_RESOURCE: Static<TagKey> = Static::new(|| "$any:resource".into());
pub static ANY_DECK: Static<TagKey> = Static::new(|| "$any:deck".into());
pub static FROM_CAMPAIGN: Static<Vec<TagKey>> = Static::new(|| {
    vec![
        ANY_NAME.clone(),
        ANY_ATTRIBUTE.clone(),
        ANY_RESOURCE.clone(),
        ANY_DECK.clone(),
    ]
});

#[derive(Debug, Serialize, Deserialize)]
pub struct TagEngine {
    pub tags: Tags,
}

impl TagEngine {
    pub fn into_combat(campaign_tags: &Self) -> Self {
        let mut tags = vec![
            Tag {
                key: ME_REF.clone(),
                value: TagValue::Tag(PLAYER.clone()),
            },
            Tag {
                key: YOU_REF.clone(),
                value: TagValue::Tag(ENEMY.clone()),
            },
        ];
        let subst_target = ANY.clone();

        FROM_CAMPAIGN.iter().for_each(|from_campaign| {
            ANY_SUBSTITUTIONS.iter().for_each(|subst| {
                let one = from_campaign.0.replace(&subst_target, subst);
                tags.append(&mut campaign_tags.find(&TagKey(one)))
            })
        });

        debug!(target:"Event/IntoCombat", "{:?}", tags);
        Self { tags: tags.into() }
    }

    pub fn handle_actions(&mut self, actions: &Vec<Action>) {
        debug!(target:"Event/Actions", "{:?}", actions);

        actions.iter().for_each(|action| match action {
            Action::Insert(tag) => {
                self.tags.insert(&tag.key, &tag.value);
            }
            Action::Remove(tag) => {
                self.tags.remove(&tag.key);
            }
            Action::Add(tag) => self.compute(tag, |cur, new| cur + new, 0.into()),
            Action::Subtract(tag) => self.compute(tag, |cur, new| cur - new, 0.into()),
            Action::Multiply(tag) => self.compute(tag, |cur, new| cur * new, 1.into()),
            Action::Divide(tag) => self.compute(tag, |cur, new| cur / new, 1.into()),
            Action::None => { /* None */ }
        });

        debug!(target:"State/Tags", "{:?}", self.tags);
    }

    fn compute(&mut self, new: &Tag, op: impl Fn(FI64, FI64) -> FI64, identity: FI64) {
        let current = match self.tags.get(&new.key) {
            Some(TagValue::Tag(tag)) => panic!(
                "Expected Number value when resolving key {:?}, but was Tag {:?}",
                new.key, tag
            ),
            Some(TagValue::Number(value)) => value,
            None => &identity,
        };

        let new_value = match &new.value {
            TagValue::Tag(tag) => match self.tags.get(tag) {
                Some(TagValue::Tag(tag)) => panic!(
                    "Expected Number value when computing key {:?}, but was Tag {:?}",
                    new.key, tag
                ),
                Some(TagValue::Number(value)) => value,
                None => &0.into(),
            },
            TagValue::Number(value) => value,
        };

        self.tags
            .insert(&new.key, &TagValue::Number(op(*current, *new_value)));
    }

    pub fn contains(&self, tag: &Tag) -> bool {
        let stored_val = match self.tags.get(&tag.key) {
            Some(TagValue::Number(value)) => value,
            Some(TagValue::Tag(tk)) => panic!(
                "Expected Number value when checking contains key {:?}, but was Tag {:?}",
                tag.key, tk
            ),
            _ => &0.into(),
        };

        let request_value = match &tag.value {
            TagValue::Tag(key) => match self.tags.get(key) {
                Some(TagValue::Number(value)) => value,
                _ => todo!(),
            },
            TagValue::Number(value) => value,
        };

        *request_value <= *stored_val
    }

    pub fn find(&self, partial_key: &TagKey) -> Vec<Tag> {
        let found = self
            .tags
            .find(partial_key)
            .map(|(k, v)| Tag::from((k.clone(), v.clone())))
            .collect();

        debug!(target:"Tags/Find", "{:?} -> {:?}", partial_key, found);
        found
    }

    pub fn satisfies(&self, predicate: &Predicate) -> bool {
        let result = match predicate {
            Predicate::Tag(tag) => self.contains(tag),
            Predicate::And(preds) => preds.iter().all(|pred| self.satisfies(pred)),
            Predicate::Or(preds) => preds.iter().any(|pred| self.satisfies(pred)),
            Predicate::Not(pred) => !self.satisfies(pred),
        };

        result
    }
}

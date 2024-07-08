use log::debug;
use serde::{Deserialize, Serialize};

use crate::core::{
    action::Action,
    predicate::Predicate,
    tags::{Tag, TagKey, TagValue, Tags, FI64},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagEngine {
    pub tags: Tags,
}

impl TagEngine {
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

        debug!(target:"Tags/Compute", "{:?} {}", new.key, op(*current, *new_value));
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
                Some(TagValue::Tag(tk)) => panic!(
                    "Expected Number value when checking comparison value for key {:?}, but was Tag {:?}",
                    key, tk
                ),
                None => panic!()
            },
            TagValue::Number(value) => value,
        };

        *request_value <= *stored_val
    }

    pub fn find(&self, partial_key: &TagKey) -> Vec<Tag> {
        self.tags.find(partial_key)
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

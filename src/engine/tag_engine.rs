use std::ops::Bound::Included;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::resource::{
    action::Action,
    predicate::Predicate,
    tag::{Tag, TagKey, TagValue, Tags},
};

static MAX_RESOLVE_DEPTH: usize = 256;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagEngine {
    pub tags: Tags,
}

impl TagEngine {
    pub fn generate() -> Self {
        TagEngine {
            tags: Tags::from(["woods:entrance:item:sword".into()]),
        }
    }

    pub fn handle_actions(&mut self, actions: &Vec<Action>) {
        debug!(target:"Event/Actions", "{:?}", actions);

        actions.iter().for_each(|action| match action {
            Action::Insert(tag) => {
                self.tags.insert(tag.0.clone(), tag.1.clone());
            }
            Action::Remove(tag) => {
                self.tags.remove(&tag.0);
            }
            Action::Add(tag) => {
                let value = self.resolve(&tag.1);
                self.compute(tag, |cur| cur + value, 0.)
            }
            Action::Subtract(tag) => {
                let value = self.resolve(&tag.1);
                self.compute(tag, |cur| cur - value, 0.)
            }
            Action::Multiply(tag) => {
                let value = self.resolve(&tag.1);
                self.compute(tag, |cur| cur * value, 1.)
            }
            Action::Divide(tag) => {
                let value = self.resolve(&tag.1);
                self.compute(tag, |cur| cur / value, 1.)
            }
            Action::None => { /* None */ }
        });

        debug!(target:"State/Tags", "{:?}", self.tags);
    }

    fn resolve(&self, tag: &TagValue) -> f64 {
        let mut next = tag.clone();
        for _ in 1..MAX_RESOLVE_DEPTH {
            match next {
                TagValue::Tag(tk) => {
                    next = self.tags.get(&tk).unwrap().clone();
                }
                TagValue::Number(val) => return val,
            }
        }
        panic!("Started from {:?} and reached max depth at {:?}", tag, next);
    }

    fn compute(&mut self, new: &Tag, op: impl Fn(f64) -> f64, identity: f64) {
        let current = self.tags.get(&new.0);
        let cur_val = current.map(|curr| self.resolve(curr)).unwrap_or(identity);

        self.tags
            .insert(new.0.clone(), TagValue::Number(op(cur_val)));
    }

    pub fn contains(&self, tag: &Tag) -> bool {
        match self.tags.get(&tag.0) {
            Some(TagValue::Number(val)) => self.resolve(&tag.1) <= *val,
            Some(TagValue::Tag(tk)) => {
                self.resolve(&tag.1) <= self.resolve(&TagValue::Tag(tk.clone()))
            }
            None => false,
        }
    }

    pub fn find(&self, partial_key: &TagKey) -> Vec<Tag> {
        let start = Included(partial_key.clone());

        let mut end_str = partial_key.clone().0;
        end_str.push('~');
        let end = Included(end_str.as_str().into());

        self.tags
            .range((start, end))
            .map(|(k, v)| Tag::from((k.clone(), v.clone())))
            .collect()
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

use std::{
    collections::{btree_set::Range, BTreeSet},
    ops::Bound,
};

use log::debug;
use serde::{Deserialize, Serialize};

use crate::resource::{
    action::Action,
    predicate::Predicate,
    tag::{Tag, Tags, TypeHint},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagEngine {
    pub tags: Tags,
}

impl TagEngine {
    pub fn generate() -> Self {
        TagEngine {
            tags: Tags(BTreeSet::from(["woods:entrance:item:sword".into()])),
        }
    }

    pub fn run_actions(&mut self, actions: &Vec<Action>) {
        actions.into_iter().for_each(|action| match action {
            Action::Insert(tag) => {
                self.tags.insert(tag.clone());
            }
            Action::Remove(tag) => {
                self.tags.remove(&tag);
            }
            Action::Add(tag) => self.compute(tag, |cur| cur + tag.as_number(), 0.),
            Action::Subtract(tag) => self.compute(tag, |cur| cur - tag.as_number(), 0.),
            Action::Multiply(tag) => self.compute(tag, |cur| cur * tag.as_number(), 0.),
            Action::Divide(tag) => self.compute(tag, |cur| cur / tag.as_number(), 0.),
            Action::None => { /* None */ }
        });

        debug!(target = "Tag"; "Run {:?}", actions);
        debug!(target = "Tag"; "State {:?}", self.tags);
    }

    fn compute(&mut self, new: &Tag, op: impl Fn(f64) -> f64, identity: f64) {
        let current = self
            .range(&new.wildcarding_numbers())
            .map(|x| x.clone())
            .collect::<Vec<_>>()
            .pop();
        let cur_val = current.as_ref().map(|c| c.as_number()).unwrap_or(identity);

        let root = match new.typehint() {
            TypeHint::Number => new.pop(),
            TypeHint::String => new.clone(),
        };
        let next = root.append(vec![op(cur_val).to_string()]);

        self.tags.remove(&new);
        if let Some(cur) = current.as_ref() {
            self.tags.remove(cur);
        }
        self.tags.insert(next);
    }

    fn range(&self, tag: &Tag) -> Range<Tag> {
        let start = tag.wildcarding_numbers();
        let end = tag
            .wildcarding_numbers()
            .append(vec![(0xff as char).into()]);
        self.tags.range((
            Bound::Included(Tag::from(start)),
            Bound::Included(Tag::from(end)),
        ))
    }

    pub fn contains(&self, tag: &Tag) -> bool {
        match tag.typehint() {
            TypeHint::String => self
                .range(tag)
                .find(|&x| x == tag || x.pop() == *tag)
                .is_some(),
            TypeHint::Number => self
                .range(tag)
                .find(|&x| x.as_number() >= tag.as_number())
                .is_some(),
        }
    }

    pub fn satisfies(&self, predicate: &Predicate) -> bool {
        let result = match predicate {
            Predicate::Tag(tag) => self.contains(&tag),
            Predicate::And(preds) => preds.iter().all(|pred| self.satisfies(pred)),
            Predicate::Or(preds) => preds.iter().any(|pred| self.satisfies(pred)),
            Predicate::Not(pred) => !self.satisfies(pred),
        };

        result
    }
}

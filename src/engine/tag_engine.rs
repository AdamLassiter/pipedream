use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::resource::{action::Action, predicate::Predicate, tag::Tags};

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

    pub fn run_actions(&mut self, actions: Vec<Action>) {
        actions.into_iter().for_each(|action| match action {
            Action::Insert(tag) => {
                self.tags.insert(tag);
            }
            Action::Remove(tag) => {
                self.tags.remove(&tag);
            }
            _ => todo!(),
        });
    }

    pub fn satisfies(&self, predicate: &Predicate) -> bool {
        match predicate {
            Predicate::Tag(tag) => self.tags.contains(tag),
            _ => todo!(),
        }
    }
}

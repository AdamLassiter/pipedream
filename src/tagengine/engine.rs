use std::collections::BTreeSet;

use crate::resource::{action::Action, tag::Tag};

pub struct TagEngine {
    tags: BTreeSet<Tag>,
}

impl TagEngine {
    pub fn new() -> Self {
        TagEngine {
            tags: BTreeSet::new(),
        }
    }

    pub fn run_actions(&mut self, actions: Vec<Action>) {
        actions.into_iter().for_each(|action| match action {
            Action::Insert(tag) => {
                self.tags.insert(tag);
            }
            Action::None => {}
            _ => {
                todo!();
            }
        })
    }
}

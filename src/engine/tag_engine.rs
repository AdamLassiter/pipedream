use std::{
    collections::{btree_set::Range, BTreeSet},
    ops::Bound,
};

use serde::{Deserialize, Serialize};

use crate::resource::{
    action::Action,
    predicate::Predicate,
    tag::{Tag, Tags},
};

const WILDCARD: char = '*';

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
            _ => todo!(),
        });
    }

    fn range(&self, tag: &Tag) -> Range<Tag> {
        if *tag.parts().last().unwrap() == WILDCARD.to_string() {
            let mut start = tag.parts();
            let mut end = tag.parts();
            start.last_mut().map(|x| *x = 0x00.to_string());
            end.last_mut().map(|x| *x = 0xff.to_string());
            self.tags.range((
                Bound::Excluded(Tag::from(start)),
                Bound::Excluded(Tag::from(end)),
            ))
        } else {
            self.tags
                .range((Bound::Included(tag), Bound::Included(tag)))
        }
    }

    pub fn contains(&self, tag: &Tag) -> bool {
        if *tag.parts().last().unwrap() == WILDCARD.to_string() {
            self.range(tag).next().is_some()
        } else {
            self.range(tag).find(|&x| x == tag).is_some()
        }
    }

    pub fn satisfies(&self, predicate: &Predicate) -> bool {
        match predicate {
            Predicate::Tag(tag) => self.contains(tag),
            _ => todo!(),
        }
    }
}

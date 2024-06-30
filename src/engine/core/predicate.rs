use serde::{Deserialize, Serialize};

use super::tag::Tag;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Predicate {
    And(Box<Vec<Predicate>>),
    Or(Box<Vec<Predicate>>),
    Not(Box<Predicate>),
    Tag(Tag),
}

impl std::fmt::Display for Predicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Self::Tag(tag) => format!("{}", tag),
                Self::And(tags) => tags
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(" AND "),
                Self::Or(tags) => tags
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(" OR "),
                Self::Not(tag) => format!("Not {}", tag),
            }
            .as_str(),
        )
    }
}

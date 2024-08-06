use serde::{Deserialize, Serialize};

use super::predicate::Predicate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    pub descriptor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
}



impl Description {
    pub fn predicated<T, U>(descriptor: U, predicate: T) -> Self
    where
        U: Into<String>,
        T: Into<Predicate>,
    {
        Self {
            descriptor: descriptor.into(),
            predicate: Some(predicate.into()),
        }
    }

    pub fn always<T>(descriptor: T) -> Self
    where
        T: Into<String>,
    {
        Description {
            predicate: None,
            descriptor: descriptor.into(),
        }
    }
}

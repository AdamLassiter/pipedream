use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::target::Target;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum FieldPlace {
    Innate,
    Deck,
    Hand,
    Discard,
    Vanish,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TargetPlace {
    pub target: Target,
    pub place: FieldPlace,
}

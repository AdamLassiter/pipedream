use serde::{Deserialize, Serialize};

use super::description::Description;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub descriptions: Vec<Description>,
}

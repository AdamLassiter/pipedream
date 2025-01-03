use serde::{Deserialize, Serialize};

use pipedream_domain::description::Description;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub descriptions: Vec<Description>,
}

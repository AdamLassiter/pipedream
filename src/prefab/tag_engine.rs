use crate::engine::{core::tag::Tags, state::tag_engine::TagEngine};

impl TagEngine {
    pub fn generate_campaign() -> Self {
        Self {
            tags: Tags::from(vec!["woods:entrance:item:sword".into()]),
        }
    }
}

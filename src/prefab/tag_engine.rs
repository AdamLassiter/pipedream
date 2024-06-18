use crate::{engine::tag_engine::TagEngine, resource::tag::Tags};

impl TagEngine {
    pub fn generate_campaign() -> Self {
        TagEngine {
            tags: Tags::from(["woods:entrance:item:sword".into()]),
        }
    }
}

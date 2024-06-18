use std::fs::File;

use crate::{engine::tag_engine::TagEngine, resource::core::tag::Tags};

impl TagEngine {
    fn dump(&self) {
        let buffer = File::create("./tags-state.yml").unwrap();
        serde_yml::to_writer(buffer, &self).unwrap();
    }

    pub fn generate_campaign() -> Self {
        let tags = TagEngine {
            tags: Tags::from(["woods:entrance:item:sword".into()]),
        };

        tags.dump();
        tags
    }
}

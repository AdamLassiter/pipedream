use std::fs::File;

use crate::{engine::tag_engine::TagEngine, resource::core::tag::Tags};

impl TagEngine {
    fn dump(&self) {
        let buffer =
            File::create("./tag-engine.yml").expect("Failed to open file for writing tag-engine data");
        serde_yml::to_writer(buffer, &self).expect("Failed to write yaml tag-engine data to file");
    }

    pub fn generate_campaign() -> Self {
        let tags = TagEngine {
            tags: Tags::from(vec!["woods:entrance:item:sword".into()]),
        };

        tags.dump();
        tags
    }
}

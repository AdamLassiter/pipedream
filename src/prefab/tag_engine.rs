use crate::engine::{core::tag::Tags, state::tag_engine::TagEngine};

impl TagEngine {
    pub fn generate_campaign() -> Self {
        Self {
            tags: Tags::from(vec![
                // World
                "woods:entrance:item:sword".into(),
                // Player
                "player:draw-count/2".into(),
                // Resources
                "player:resource:health/20".into(),
                "player:resource:stamina/20".into(),
                "player:resource:mana/20".into(),
                "player:resource:faith/20".into(),
                // Deck
                "player:deck:Anathema Device".into(),
                "player:deck:Bag of Endless Bags".into(),
                "player:deck:Regular Punch".into(),
                "player:deck:Consecutive Regular Punches".into(),
            ]),
        }
    }
}

use crate::engine::combat::npc::{Npc, Npcs};

impl Npcs {
    pub fn generate() -> Self {
        Self::generate_vec().into()
    }

    fn generate_vec() -> Vec<Npc> {
        vec![
            Npc {
                name: "Slightly Larger Dave".into(),
                tags: vec!["enemy:name:Slightly Larger Dave".into()].into(),
            },
            Npc {
                name: "Dave".into(),
                tags: vec!["enemy:name:Dave".into()].into(),
            },
        ]
    }
}

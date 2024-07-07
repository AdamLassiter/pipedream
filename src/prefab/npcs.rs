use crate::engine::combat::npc::{Npc, Npcs};

impl Npcs {
    pub fn generate() -> Self {
        Self::generate_vec().into()
    }

    fn generate_vec() -> Vec<Npc> {
        vec![
            Npc {
                name: "Slightly Larger Dave".into(),
                tags: vec![
                    "$enemy:name:Slightly Larger Dave".into(),
                    "$enemy:resource:health/10".into(),
                    "$enemy:resource:stamina/10".into(),
                    "$enemy:resource:mana/10".into(),
                    "$enemy:resource:faith/10".into(),
                ]
                .into(),
            },
            Npc {
                name: "Dave".into(),
                tags: vec![
                    "$enemy:name:Dave".into(),
                    "$enemy:resource:health/1".into(),
                    "$enemy:resource:stamina/1".into(),
                    "$enemy:resource:mana/1".into(),
                    "$enemy:resource:faith/1".into(),
                ]
                .into(),
            },
        ]
    }
}

use pipedream_engine::{
    combat::npc::{Npc, Npcs},
    core::tags::Tags,
};

use crate::{Buildable, Generatable};

impl Generatable for Npcs {
    fn generate() -> Self {
        generate_vec().into()
    }
}

fn generate_vec() -> Vec<Npc> {
    vec![
        Npc {
            name: "Slightly Larger Dave".into(),
            tags: Tags::build(vec![
                "enemy:name:Slightly Larger Dave".into(),
                "enemy:resource:health=10".into(),
                "enemy:resource:stamina=10".into(),
                "enemy:resource:mana=10".into(),
                "enemy:resource:faith=10".into(),
            ]),
        },
        Npc {
            name: "Dave".into(),
            tags: Tags::build(vec![
                "enemy:name:Dave".into(),
                "enemy:image:resources/rpg/demon-avatar-icons-pixel-art-64x64/png/transperent/icon42.png".into(),
                "enemy:resource:health=1".into(),
                "enemy:resource:stamina=1".into(),
                "enemy:resource:mana=1".into(),
                "enemy:resource:faith=1".into(),
            ]),
        },
    ]
}

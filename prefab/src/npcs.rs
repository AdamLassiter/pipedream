use pipedream_domain::character::{Npc, Npcs};
use pipedream_engine::tag::Tags;

use crate::{Buildable, Generatable};

impl Generatable for Npcs {
    fn generate() -> Self {
        generate_vec().into()
    }
}

pub struct Pc(pub Npc);

pub fn generate_player() -> Pc {
    Pc(Npc {
        name: "Plae-Yerr".into(),
        image: "resources/avatars/fairy/png/transperent/icon24.png".into(),
        tags: Tags::build(vec![
            "player:name:Plae-Yerr".into(),
            "player:draw:count=4".into(),
            // Resources
            "player:resource:health=20".into(),
            "player:resource:stamina=20".into(),
            "player:resource:mana=20".into(),
            "player:resource:faith=20".into(),
            // Deck
            "player:deck:Anathema Device".into(),
            "player:deck:Bag of Endless Bags".into(),
            "player:deck:Regular Punch=3".into(),
            "player:deck:Immolate".into(),
        ]),
    })
}

fn generate_vec() -> Vec<Npc> {
    vec![
        generate_player().0,
        Npc {
            name: "Slightly Larger Dave".into(),
            image: "resources/rpg/demon/png/transperent/icon2.png".into(),
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
            image: "resources/rpg/demon/png/transperent/icon3.png".into(),
            tags: Tags::build(vec![
                "enemy:name:Dave".into(),
                "enemy:resource:health=1".into(),
                "enemy:resource:stamina=1".into(),
                "enemy:resource:mana=1".into(),
                "enemy:resource:faith=1".into(),
            ]),
        },
    ]
}

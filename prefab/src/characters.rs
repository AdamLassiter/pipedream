use pipedream_domain::{
    character::{Character, CharacterDao},
    player::{Player::Human, PlayerCharacter, PlayerCharacterDao},
    stats::{Resource, Stats},
};
use pipedream_engine::image::Image;

use crate::Prefabricated;

impl Prefabricated for Character {
    fn initialise(conn: &rusqlite::Connection) {
        let human = Self {
            name: "Plae-Yerr".into(),
            image: Image::new("resources/avatars/fairy/png/transperent/icon24.png"),
            stats: Stats::default(),
        };
        let characters = vec![
            Self {
                name: "Dave".into(),
                image: Image::new("resources/rpg/demon/png/transperent/icon3.png"),
                stats: {
                    let mut stats = Stats::default();
                    stats.resources.insert(Resource::Health, 1);
                    stats.max_resources.insert(Resource::Health, 1);
                    stats
                },
            },
            Self {
                name: "Slightly Larger Dave".into(),
                image: Image::new("resources/rpg/demon/png/transperent/icon2.png"),
                stats: Stats::default(),
            },
        ];

        CharacterDao::drop_table(conn).expect("Failed to drop table for Character");
        CharacterDao::create_table(conn).expect("Failed to create table for Character");
        let human_id = CharacterDao::from(human).insert(conn).unwrap();
        characters.into_iter().for_each(|c| {
            CharacterDao::from(c)
                .insert(conn)
                .expect("Failed to prefabricate Character");
        });

        PlayerCharacterDao::drop_table(conn).expect("Failed to drop table for PlayerCharacter");
        PlayerCharacterDao::create_table(conn).expect("Failed to create table for PlayerCharacter");
        PlayerCharacterDao::from(PlayerCharacter {
            player: Human,
            character: human_id,
        })
        .insert(conn)
        .expect("Failed to prefabricate PlayerCharacter");
    }
}

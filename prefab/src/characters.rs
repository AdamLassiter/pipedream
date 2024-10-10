use crate::Prefabricated;
use pipedream_domain::{
    card::CardDao,
    character::{Character, CharacterDao},
    image::Image,
    player::{Player::Human, PlayerCharacter, PlayerCharacterDao},
    stats::{Resource, Stats}, target::TargetCharacterDao,
};

impl Prefabricated for Character {
    fn initialise(conn: &rusqlite::Connection) {
        let find = |card| {
            CardDao::select_title(conn, &String::from(card))
                .ok()
                .and_then(|mut rs| rs.pop())
                .and_then(|r| {
                    let (id, _) = r.into();
                    id
                })
                .unwrap_or_else(|| panic!("Failed to find card {card}"))
        };
        let human = Self {
            name: "Plae-Yerr".into(),
            image: Image::new("resources/avatars/fairy/png/transperent/icon24.png"),
            stats: Stats::default(),
            cards: vec![
                find("Immolate"),
                find("Anathema Device"),
                find("Regular Punch"),
            ],
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
                cards: vec![find("Regular Punch")],
            },
            Self {
                name: "Slightly Larger Dave".into(),
                image: Image::new("resources/rpg/demon/png/transperent/icon2.png"),
                stats: Stats::default(),
                cards: vec![find("Regular Punch")],
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

        TargetCharacterDao::drop_table(conn).expect("Failed to drop table for TargetCharacter");
        TargetCharacterDao::create_table(conn).expect("Failed to create table for TargetCharacter");

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

use crate::{combat_world::COMBAT_INIT, Prefabricated};
use pipedream_domain::{
    card::PlacedCardDao, character::CharacterDao, field::FieldPlace, player::Player,
    player::PlayerCharacterDao,
};
use pipedream_engine::{
    action::Action,
    choice::{Card, CardDao, Choices},
    command::UiMode,
    description::Description,
    effect::{Effect, Transition},
    image::Image,
    location::Location,
    predicate::Predicate,
    scene::Scene,
    state::{State, StateDao},
};
use rusqlite::Connection;

fn player_has_card<T>(player: Player, card_name: T) -> Predicate
where
    T: Into<String>,
{
    let players = PlayerCharacterDao::table_name();
    let placed_cards = PlacedCardDao::table_name();
    let cards = CardDao::table_name();
    let has = format!(
        "select count(*)
        from {players} as player
        inner join {placed_cards} as placed on placed.character = player.character
        inner join {cards} as card on placed.card = card.id
        where player.player = :player
        and card.title = :card_name;"
    );
    let params = vec![
        (":player", serde_json::to_string(&player)),
        (":card_name", Ok(card_name.into())),
    ];
    Predicate::parameterised(has, params)
}

fn give_player_card<T>(player: Player, card_name: T) -> Action
where
    T: Into<String>,
{
    let players = PlayerCharacterDao::table_name();
    let placed_cards = PlacedCardDao::table_name();
    let cards = CardDao::table_name();
    let give = format!(
        "insert into {placed_cards} (character, card, place)
        select player.character, card.id, place.*
        from (
            select *
            from {players}
            where player = :player
            limit 1
        ) as player,
        (
            select *
            from {cards}
            where title = :card_name
            limit 1
        ) as card,
        (
            values (:deck)
        ) as place;"
    );
    let params = vec![
        (":deck", serde_json::to_string(&FieldPlace::Deck)),
        (":player", serde_json::to_string(&player)),
        (":card_name", Ok(card_name.into())),
    ];
    Action::parameterised(give, params)
}

fn take_player_card<T>(player: Player, card_name: T) -> Action
where
    T: Into<String>,
{
    let players = PlayerCharacterDao::table_name();
    let placed_cards = PlacedCardDao::table_name();
    let cards = CardDao::table_name();
    let take = format!(
        "delete from {placed_cards}
        where id in
        (
            select placed.id
            from {players} as player
            inner join {placed_cards} as placed on placed.character = player.character
            inner join {cards} as card on placed.card = card.id
            where player.player = :player
            and card.title = :card_name
            limit 1
        );"
    );
    let params = vec![
        (":player", serde_json::to_string(&player)),
        (":card_name", Ok(card_name.into())),
    ];
    Action::parameterised(take, params)
}

fn enter_combat<T>(enemy_name: T) -> Effect
where
    T: Into<String>,
{
    let players = PlayerCharacterDao::table_name();
    let characters = CharacterDao::table_name();
    let placed_cards = PlacedCardDao::table_name();
    let cards = CardDao::table_name();
    let enemy = format!(
        "insert into {players} (player, character)
        select player.*, character.id
        from (
            values (:cpu)
        ) as player,
        (
            select *
            from {characters} as character
            where character.name = :enemy_name
        ) as character;"
    );
    let cards = format!(
        "insert into {placed_cards} (character, card, place)
        select character, card, place.*
        from (
            select player.character as character, card.id as card
            from {players} as player
            inner join {placed_cards} as placed on placed.character = player.character
            inner join {cards} as card on placed.card = card.id
            where player.player = :cpu
        ),
        (
            values (:deck)
        ) as place;"
    );
    let enemy_params = vec![
        (":cpu", serde_json::to_string(&Player::Cpu)),
        (":enemy_name", Ok(enemy_name.into())),
    ];
    let cards_params = vec![
        (":cpu", serde_json::to_string(&Player::Cpu)),
        (":deck", serde_json::to_string(&FieldPlace::Deck)),
    ];
    let actions = vec![
        Action::parameterised(enemy, enemy_params),
        Action::parameterised(cards, cards_params),
    ];
    let transition = Transition::Enter(COMBAT_INIT.clone());
    Effect {
        transition,
        actions,
    }
}

impl Prefabricated for State {
    fn initialise(conn: &Connection) {
        let states = vec![
            State {
                location: Location::new("woods:entrance"),
                scene: Scene {
                    descriptions: vec![
                        Description::always("You are in <green the woods>"),
                        Description::always("There is a mysterious moss-covered shop in a small grove"),
                        Description::predicated(
                            "You see a shiny sword lodged in a stone",
                            player_has_card(Player::Human, "Mossy Sword").inverse()
                        ),
                    ],
                },
                choices: Choices::cards(vec![
                    Card {
                        title: "Pick up the sword".into(),
                        image: Image::new("resources/hi-res/sword/png/without_shadow/7.png"),
                        predicate: Some(player_has_card(Player::Human, "Mossy Sword").inverse()),
                        effect: Effect::actions(vec![give_player_card(Player::Human, "Mossy Sword")]),
                        ..Default::default()
                    },
                    Card {
                        title: "Go into the shop".into(),
                        image: Image::new("resources/scenery/glade/png/objects_separated/assets_no_shadow/house1.png"),
                        effect: Effect::transition(Transition::Enter(Location::new("ephemeral:shop"))),
                        ..Default::default()
                    },
                    Card {
                        title: "Go deeper into the woods".into(),
                        image: Image::new("resources/scenery/forest/png/assets_no_shadow/luminous_tree1.png"),
                        effect: Effect::transition(Transition::Goto(Location::new("woods:depths"))),
                        ..Default::default()
                    },
                ]),
                ui_mode: UiMode::Campaign,
            },
            State {
                location: Location::new("woods:depths"),
                scene: Scene {
                    descriptions: vec![Description::always("You are lost in <green the woods>")],
                },
                choices: Choices::cards(vec![
                    Card {
                        title: "Go deeper into the woods".into(),
                        image: Image::new("resources/scenery/forest/png/assets_no_shadow/luminous_tree2.png"),
                        effect: Effect::transition(Transition::Goto(Location::new("woods:depths"))),
                        ..Default::default()
                    },
                    Card {
                        title: "Battle <red inner demons>".into(),
                        image: Image::new("resources/avatars/demon-hires/png/transperent/icon42.png"),
                        effect: enter_combat("Dave"),
                        ..Default::default()
                    },
                ]),
                ui_mode: UiMode::Campaign,
            },
            State {
                location: Location::new("ephemeral:shop"),
                scene: Scene {
                    descriptions: vec![
                        Description::always("The shop is cozy, and staffed by a weathered crone"),
                        Description::predicated(
                            "Her eyes keep flitting to the sword at your side",
                            player_has_card(Player::Human, "Mossy Sword"),
                        ),
                    ],
                },
                choices: Choices::cards(vec![
                    Card {
                        title: "Leave the shop".into(),
                        image: Image::new("resources/avatars/dark-elf-2/png/dark_elves_faces_transperent/character6_face1.png"),
                        effect: Effect::transition(Transition::Leave),
                        ..Default::default()
                    },
                    Card {
                        title: "Trade a sword for two swords".into(),
                        image: Image::new("resources/avatars/dark-elf-2/png/dark_elves_faces_transperent/character6_face2.png"),
                        predicate: Some(player_has_card(Player::Human, "Mossy Sword")),
                        effect: Effect::actions(vec![
                            take_player_card(Player::Human, "Mossy Sword"),
                            give_player_card(Player::Human, "Mossy Sword"),
                            give_player_card(Player::Human, "Mossy Sword"),
                        ]),
                        ..Default::default()
                    },
                ]),
                ui_mode: UiMode::Campaign,
            },
        ];

        StateDao::drop_table(conn).expect("Failed to drop table for State");
        StateDao::create_table(conn).expect("Failed to create table for State");
        states.into_iter().for_each(|s| {
            StateDao::from(s)
                .insert(conn)
                .expect("Failed to prefabricate State");
        });
    }
}

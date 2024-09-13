use crate::{combat_world::COMBAT_INIT, Prefabricated, Static};
use pipedream_domain::{card::PlacedCard, character::PlayerCharacter, field::FieldPlace, player::Player};
use pipedream_engine::{
    action::Action, choice::{Card, Choice, Choices}, description::Description, effect::{Effect, Transition}, image::Image, location::Location, predicate::Predicate, scene::Scene, state::State
};
use rusqlite::Connection;

fn player_has_card<T>(player: Player, card_name: T) -> Predicate where T: Into<String> {
    let players = PlayerCharacter::table_name();
    let placed_cards = PlacedCard::table_name();
    let cards = Card::table_name();
    let sql = format!("
        select count(*)
        from {players} player
            right join {placed_cards} placed on (placed.data)->character = (player.data)->character
            left  join {cards}        card   on (placed.data)->card = card.id
        where (player.data)->player = :player
        where (card.data)->title = :card_name;
    ");
    let params = vec![
        (":player", serde_json::to_value(player)),
        (":card_name", serde_json::to_value(card_name.into()))
    ];
    Predicate::parameterised(sql, params)
}

fn give_player_card<T>(player: Player, card_name: T) -> Action where T: Into<String> {
    let players = PlayerCharacter::table_name();
    let placed_cards = PlacedCard::table_name();
    let cards = Card::table_name();
    let sql = format!("
        insert into {placed_cards} ( character, card, place )
        select (player.data)->character, card.id, :deck
        from {players} player
            left join {cards} card on (placed.data)->card = card.id
        where (player.data)->player = :player
        where (card.data)->title = :card_name;
    ");
    let params = vec![
        (":deck", serde_json::to_value(FieldPlace::Deck)),
        (":player", serde_json::to_value(player)),
        (":card_name", serde_json::to_value(card_name.into()))
    ];
    Action::parameterised(vec![sql], params)
}

fn take_player_card<T>(player: Player, card_name: T) -> Action where T: Into<String> {
    let players = PlayerCharacter::table_name();
    let placed_cards = PlacedCard::table_name();
    let cards = Card::table_name();
    let sql = format!("
        delete from {placed_cards}
        where id in
        (
            select (player.data)->character, card.id, :deck
            from {players} player
                left join {cards} card on (placed.data)->card = card.id
            where (player.data)->player = :player
            where (card.data)->title = :card_name
            limit 1
        );
    ");
    let params = vec![
        (":deck", serde_json::to_value(FieldPlace::Deck)),
        (":player", serde_json::to_value(player)),
        (":card_name", serde_json::to_value(card_name.into()))
    ];
    Action::parameterised(vec![sql], params)
}

fn enter_combat<T>(enemy_name: T) -> Effect where T: Into<String> {
    let players = PlayerCharacter::table_name();
    let placed_cards = PlacedCard::table_name();
    let cards = Card::table_name();
    let sql = format!("
        insert into {players} ( character, card, place )
        select (player.data)->character, card.id, :deck
        from {players} player
            left join {cards} card on (placed.data)->card = card.id
        where (player.data)->player = :player
        where (card.data)->title = :card_name;
    ");
    let params = vec![
        (":deck", serde_json::to_value(FieldPlace::Deck)),
        // (":player", serde_json::to_value(player)),
        // (":card_name", serde_json::to_value(card_name.into()))
    ];
    let actions = vec![Action::parameterised(vec![sql], params)];
    let transition = Transition::Enter(COMBAT_INIT.clone());
    Effect {transition, actions}
}

impl Prefabricated for State {
    fn initialise(conn: &Connection) {
        vec![
            State {
                location: Location::campaign("woods:entrance"),
                scene: Scene {
                    descriptions: vec![
                        Description::always("You are in <green the woods>"),
                        Description::always("There is a mysterious moss-covered shop in a small grove"),
                        Description::predicated(
                            "You see a shiny sword lodged in a stone",
                            player_has_card(Player::Human, "Mossy Sword").inverse()
                        )
                            .into(),
                    ],
                },
                choices: Choices::cards(vec![
                    Card {
                        title: "Pick up the sword".into(),
                        image: Image::new("resources/hi-res/sword/png/without_shadow/7.png"),
                        predicate: Some(player_has_card(Player::Human, "Mossy Sword").inverse()),
                        effect: Effect {
                            transition: Transition::None,
                            actions: vec![give_player_card(Player::Human, "Mossy Sword")],
                        },
                        ..Default::default()
                    },
                    Card {
                        title: "Go into the shop".into(),
                        image: Image::new("resources/scenery/glade/png/objects_separated/assets_no_shadow/house1.png"),
                        effect: Effect {
                            transition: Transition::Enter(Location::campaign("ephemeral:shop")),
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                    Card {
                        title: "Go deeper into the woods".into(),
                        image: Image::new("resources/scenery/forest/png/assets_no_shadow/luminous_tree1.png"),
                        effect: Effect {
                            transition: Transition::Goto(Location::campaign("woods:depths")),
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                ])
                .into(),
            },
            State {
                location: Location::campaign("woods:depths"),
                scene: Scene {
                    descriptions: vec![Description::always("You are lost in <green the woods>")],
                },
                choices: Choices::cards(vec![
                    Card { 
                        title: "Go deeper into the woods".into(),
                        image: Image::new("resources/scenery/forest/png/assets_no_shadow/luminous_tree2.png"),
                        effect: Effect {
                            transition: Transition::Goto(Location::campaign("woods:depths")),
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                    Card { 
                        title: "Battle <red inner demons>".into(),
                        image: Image::new("resources/avatars/demon-hires/png/transperent/icon42.png"),
                        effect: enter_combat("Dave"),
                        ..Default::default()
                    },
                ]),
            },
            State {
                location: Location::campaign("ephemeral:shop"),
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
                        image: Image::new("resources/avatars/dark-elf-2/png/dark elves_faces_transperent/character6_face1.png"),
                        effect: Effect {
                            transition: Transition::Leave,
                            actions: vec![],
                        },
                        ..Default::default()
                    },
                    Card {
                        title: "Trade a sword for two swords".into(),
                        image: Image::new("resources/avatars/dark-elf-2/png/dark elves_faces_transperent/character6_face2.png"),
                        predicate: Some(player_has_card(Player::Human, "Mossy Sword")),
                        effect: Effect {
                            transition: Transition::None,
                            actions: vec![
                                take_player_card(Player::Human, "Mossy Sword"),
                                give_player_card(Player::Human, "Mossy Sword"),
                                give_player_card(Player::Human, "Mossy Sword"),
                            ],
                        },
                        ..Default::default()
                    },
                ]),
            },
        ].into_iter()
        .for_each(|state| {state.insert(&conn).unwrap();});
    }
}

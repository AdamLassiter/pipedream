use crate::Prefabricated;
use pipedream_domain::{
    action::Action,
    card::{Card, CardDao, PlacedCardDao},
    character::CharacterDao,
    choice::Choice,
    description::Description,
    effect::Effect,
    field::FieldPlace,
    image::Image,
    predicate::Predicate,
    stats::Resource,
    target::{Target, TargetCharacter, TargetCharacterDao},
};

fn target_has_resource(target: Target, resource: Resource, amount: i64) -> Predicate {
    let targets = TargetCharacterDao::table_name();
    let characters = CharacterDao::table_name();
    let has = format!(
        "select count(*)
        from {targets} as target
        inner join {characters} as character on target.character = character.id
        where json_extract(character.stats, '$.resources.{resource}') >= {amount}
        and target.target = :target;"
    );
    let params = vec![(":target", serde_json::to_string(&target))];
    Predicate::parameterised(has, params)
}

fn modify_target_resource(conn: &rusqlite::Connection, target: Target, resource: Resource, amount: i64) -> Action {
    let characters = CharacterDao::table_name();
    let (_id, TargetCharacter {character, ..}) = TargetCharacter::get_target(conn, &target);
    let character_id = character.0;
    let modify = format!(
        "update {characters}
        set stats = json_set(stats, '$.resources.{resource}', json_extract(stats, '$.resources.{resource}') + {amount})
        where id = {character_id};"
    );
    Action::pure(modify)
}

fn set_expr_target_resource<T>(conn: &rusqlite::Connection, target: Target, resource: Resource, expr: T) -> Action
where
    T: Into<String>,
{
    let expr = expr.into();
    let characters = CharacterDao::table_name();
    let (_id, TargetCharacter {character, ..}) = TargetCharacter::get_target(conn, &target);
    let character_id = character.0;
    let modify = format!(
        "update {characters}
        set stats = json_set(stats, '$.resources.{resource}', {expr})
        where id = {character_id};"
    );
    Action::pure(modify)
}

impl Prefabricated for Card {
    fn initialise(conn: &rusqlite::Connection) {
        let cards = vec![
            Self::new(
                Choice {
                    title: "Anathema Device".into(),
                    image: Image::new("resources/legacy/tile269.png"),
                    details: vec![Description::always(
                        "Apply <blue 0 anathema> [Self]",
                    )],
                    cost: Some("<blue 10 mana>".into()),
                    predicate: Some(target_has_resource(Target::Me, Resource::Mana, 10)),
                    effect: Effect::actions(vec![
                        modify_target_resource(conn, Target::Me, Resource::Mana, -5),
                        modify_target_resource(conn, Target::Me, Resource::Mana, -5),
                    ]),
                    ..Default::default()
                },
                FieldPlace::Deck,
            ),
            Self::new(
                Choice {
                    title: "Regular Punch".into(),
                    image: Image::new("resources/legacy/tile095.png"),
                    details: vec![Description::always("Damage <red 2 health> [Enemy]")],
                    cost: Some("<green 1 stamina>".into()),
                    predicate: Some(target_has_resource(Target::Me, Resource::Stamina, 1)),
                    effect: Effect::actions(vec![
                        modify_target_resource(conn, Target::Me, Resource::Stamina, -1),
                        modify_target_resource(conn, Target::You, Resource::Health, -2),
                    ]),
                    ..Default::default()
                },
                FieldPlace::Deck,
            ),
            Self::new(
                Choice {
                    title: "Mossy Sword".into(),
                    image: Image::new("resources/legacy/tile083.png"),
                    details: vec![Description::always("Damage <red 5 health> [Enemy]")],
                    cost: Some("<green 2 stamina>".into()),
                    predicate: Some(target_has_resource(Target::Me, Resource::Stamina, 2)),
                    effect: Effect::actions(vec![
                        modify_target_resource(conn, Target::Me, Resource::Stamina, -2),
                        modify_target_resource(conn, Target::You, Resource::Health, -2),
                    ]),
                    ..Default::default()
                },
                FieldPlace::Deck,
            ),
            Self::new(
                Choice {
                    title: "Immolate".into(),
                    image: Image::new("resources/legacy/tile009.png"),
                    details: vec![
                        Description::always("Damage <red 100% self health> [Enemy]"),
                        Description::always("Damage <green 100% self stamina> [Enemy]"),
                    ],
                    cost: Some("<red 100% health>, <green 100% stamina>".into()),
                    predicate: Some(target_has_resource(Target::Me, Resource::Stamina, 1)),
                    effect: Effect::actions(vec![
                        set_expr_target_resource(
                            conn,                        
                            Target::Me,
                            Resource::Health,
                            format!(
                                "cast(json_extract(character.stats, '$.resources.{health}') * 0.99 as int)",
                                health = Resource::Health,
                            ),
                        ),
                        set_expr_target_resource(
                            conn,                        
                            Target::Me,
                            Resource::Stamina,
                            format!(
                                "cast(json_extract(character.stats, '$.resources.{stamina}') * 0.99 as int)",
                                stamina = Resource::Stamina,
                            ),
                        ),
                        set_expr_target_resource(
                            conn,                        
                            Target::You,
                            Resource::Health,
                            format!(
                                "cast(json_extract(character.stats, '$.resources.{health}') * 0.99 as int)",
                                health = Resource::Health,
                            ),
                        ),
                    ]),
                    ..Default::default()
                },
                FieldPlace::Deck,
            ),
        ];

        CardDao::drop_table(conn).expect("Failed to drop table for Card");
        CardDao::create_table(conn).expect("Failed to create table for Card");
        cards.into_iter().for_each(|c| {
            CardDao::from(c)
                .insert(conn)
                .expect("Failed to prefabricate Card");
        });

        PlacedCardDao::drop_table(conn).expect("Failed to drop table for PlacedCard");
        PlacedCardDao::create_table(conn).expect("Failed to create table for PlacedCard");
    }
}

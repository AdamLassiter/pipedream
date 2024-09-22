use crate::Prefabricated;
use pipedream_domain::{
    card::PlacedCardDao,
    character::CharacterDao,
    stats::Resource,
    target::{Target, TargetCharacterDao},
};
use pipedream_engine::{
    action::Action,
    choice::{Card, CardDao},
    description::Description,
    effect::Effect,
    image::Image,
    predicate::Predicate,
};

fn target_has_resource(target: Target, resource: Resource, amount: i64) -> Predicate {
    let targets = TargetCharacterDao::table_name();
    let characters = CharacterDao::table_name();
    let resource = serde_json::to_string(&&resource).expect("Failed to serialize Resource enum");
    let has = format!(
        "select count(*)
        from {targets} as target
        inner join {characters} as character on target.character = character.id
        where (character.stats)->'$.resources[{resource}]' > :amount
        and target.target = :target;"
    );
    let params = vec![
        (":target", serde_json::to_string(&target)),
        (":amount", Ok(amount.to_string())),
    ];
    Predicate::parameterised(has, params)
}

fn modify_target_resource(target: Target, resource: Resource, amount: i64) -> Action {
    let targets = TargetCharacterDao::table_name();
    let characters = CharacterDao::table_name();
    let resource = serde_json::to_string(&&resource).expect("Failed to serialize Resource enum");
    let modify = format!(
        "update {characters}
        set character.stats = json_set(character.stats, '$.resources[{resource}]', :amount)
        from {targets} as target
        inner join {characters} as character on target.character = character.id
        where (character.stats)->'$.resources[{resource}]' > :amount
        and target.target = :target;"
    );
    let params = vec![
        (":target", serde_json::to_string(&target)),
        (":amount", Ok(amount.to_string())),
    ];
    Action::parameterised(modify, params)
}

fn modify_expr_target_resource<T>(target: Target, resource: Resource, expr: T) -> Action
where
    T: Into<String>,
{
    let expr = expr.into();
    let targets = TargetCharacterDao::table_name();
    let characters = CharacterDao::table_name();
    let resource = serde_json::to_string(&&resource).expect("Failed to serialize Resource enum");
    let modify = format!(
        "update {characters}
        set character.stats = json_set(character.stats, '$.resources[{resource}]', {expr})
        from {targets} as target
        inner join {characters} as character on target.character = character.id
        where (character.stats)->'$.resources[{resource}]' > {expr}
        and target.target = :target;"
    );
    let params = vec![(":target", serde_json::to_string(&target))];
    Action::parameterised(modify, params)
}

impl Prefabricated for Card {
    fn initialise(conn: &rusqlite::Connection) {
        let cards = vec![
            Self {
                title: "Anathema Device".into(),
                image: Image::new("resources/legacy/tile269.png"),
                details: vec![Description::always("Apply <blue 0 anathema> [Self]")],
                cost: Some("<blue 10 mana>".into()),
                predicate: Some(target_has_resource(Target::Me, Resource::Mana, 10)),
                effect: Effect::actions(vec![
                    modify_target_resource(Target::Me, Resource::Mana, -5),
                    modify_target_resource(Target::Me, Resource::Mana, -5),
                ]),
            },
            Self {
                title: "Regular Punch".into(),
                image: Image::new("resources/legacy/tile095.png"),
                details: vec![Description::always("Damage <red 2 health> [Enemy]")],
                cost: Some("<green 1 stamina>".into()),
                predicate: Some(target_has_resource(Target::Me, Resource::Stamina, 1)),
                effect: Effect::actions(vec![
                    modify_target_resource(Target::Me, Resource::Stamina, -1),
                    modify_target_resource(Target::You, Resource::Health, -2),
                ]),
            },
            Self {
                title: "Mossy Sword".into(),
                image: Image::new("resources/legacy/tile083.png"),
                details: vec![Description::always("Damage <red 5 health> [Enemy]")],
                cost: Some("<green 2 stamina>".into()),
                predicate: Some(target_has_resource(Target::Me, Resource::Stamina, 2)),
                effect: Effect::actions(vec![
                    modify_target_resource(Target::Me, Resource::Stamina, -2),
                    modify_target_resource(Target::You, Resource::Health, -2),
                ]),
            },
            Self {
                title: "Immolate".into(),
                image: Image::new("resources/legacy/tile095.png"),
                details: vec![
                    Description::always("Damage <red 100% self health> [Enemy]"),
                    Description::always("Damage <green 100% self stamina> [Enemy]"),
                ],
                cost: Some("<red 100% health>, <green 100% stamina>".into()),
                predicate: Some(target_has_resource(Target::Me, Resource::Stamina, 1)),
                effect: Effect::actions(vec![
                    modify_expr_target_resource(
                        Target::Me,
                        Resource::Health,
                        format!(
                            "round((character.stats)->'$.resources[{health}]' * 0.99)",
                            health = serde_json::to_string(&&Resource::Health)
                                .expect("Faield to serialize Resource enum")
                        ),
                    ),
                    modify_expr_target_resource(
                        Target::Me,
                        Resource::Stamina,
                        format!(
                            "round((character.stats)->'$.resources[{stamina}]' * 0.99)",
                            stamina = serde_json::to_string(&&Resource::Stamina)
                                .expect("Faield to serialize Resource enum")
                        ),
                    ),
                    modify_expr_target_resource(
                        Target::You,
                        Resource::Health,
                        format!(
                            "round((character.stats)->'$.resources[{health}]' * 0.99)",
                            health = serde_json::to_string(&&Resource::Health)
                                .expect("Faield to serialize Resource enum")
                        ),
                    ),
                ]),
            },
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

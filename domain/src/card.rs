use rusqlite::Connection;
use rusqlite_orm::orm_bind;

use pipedream_engine::{action::Action, choice::CardId};

use crate::{character::CharacterId, field::FieldPlace};

#[derive(Clone, Debug)]
#[orm_bind ({character: "$.character", place: "$.place"}, [ (character, place) ])]
pub struct PlacedCard {
    pub character: CharacterId,
    pub card: CardId,
    pub place: FieldPlace,
}

impl PlacedCard {
    pub fn get_placed_cards(
        conn: &Connection,
        character: &CharacterId,
        place: &FieldPlace,
    ) -> Vec<(PlacedCardId, Self)> {
        PlacedCard::query_by_character_and_place(conn, character, place)
            .ok()
            .unwrap_or_else(|| panic!("Failed to find PlacedCard for {:?} and {:?}", character, place))
    }

    pub fn update_placed_cards(
        conn: &Connection,
        character: &CharacterId,
        place: &FieldPlace,
        update: impl Fn(Vec<Self>) -> Vec<Self>,
    ) -> Vec<Action> {
        let placed_cards = PlacedCard::query_by_character_and_place(conn, character, place)
            .ok()
            .unwrap_or_else(|| {
                panic!("Failed to find PlacedCard for {:?} and {:?}", character, place)
            });

        let (ids, cards): (Vec<_>, Vec<_>) = placed_cards.into_iter().unzip();
        let cards = update(cards);
        let placed_cards = ids.into_iter().zip(cards).collect::<Vec<_>>();

        placed_cards
            .into_iter()
            .map(|(id, card)| Action {
                sql_batch: vec![PlacedCard::update_sql().to_string()],
                params: vec![
                    (
                        ":id".to_string(),
                        serde_json::to_value(id.0).expect("Failed to serialize Id to json"),
                    ),
                    (
                        ":data".to_string(),
                        serde_json::to_value(card).expect("Failed to serialize PlacedCard to json"),
                    ),
                ],
            })
            .collect()
    }
}

use rusqlite::Connection;
use rusqlite_orm::orm_bind;

use pipedream_engine::{action::Action, choice::CardId};

use crate::{field::FieldPlace, player::Player};

#[derive(Clone, Debug)]
#[orm_bind ({player: "$.player", place: "$.place"}, [ (player, place) ])]
pub struct PlacedCard {
    pub player: Player,
    pub place: FieldPlace,
    pub card: CardId,
}

impl PlacedCard {
    pub fn get_placed_cards(
        conn: &Connection,
        player: &Player,
        place: &FieldPlace,
    ) -> Vec<(PlacedCardId, Self)> {
        PlacedCard::query_by_player_and_place(conn, player, place)
            .ok()
            .unwrap_or_else(|| panic!("Failed to find PlacedCard for {:?} and {:?}", player, place))
    }

    pub fn update_placed_cards(
        conn: &Connection,
        player: &Player,
        place: &FieldPlace,
        update: impl Fn(Vec<Self>) -> Vec<Self>,
    ) -> Vec<Action> {
        let placed_cards = PlacedCard::query_by_player_and_place(conn, player, place)
            .ok()
            .unwrap_or_else(|| {
                panic!("Failed to find PlacedCard for {:?} and {:?}", player, place)
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

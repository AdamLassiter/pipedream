use rusqlite::Connection;
use rusqlite_orm::orm_autobind;

use crate::{action::Action, character::CharacterId, choice::Choice, field::FieldPlace};

#[derive(Debug, Clone)]
#[orm_autobind]
pub struct Card {
    title: String,
    pub choice: Choice,
    pub starts: FieldPlace,
}
impl Card {
    pub fn new(choice: Choice, starts: FieldPlace) -> Self {
        Self {
            title: choice.title.clone(),
            choice,
            starts,
        }
    }

    pub fn get_card(conn: &Connection, card_id: &CardId) -> Option<Self> {
        CardDao::select_id(conn, card_id)
            .ok()
            .and_then(|mut cards| cards.pop())
            .map(|card| card.into())
    }

    pub fn get_card_title(
        conn: &Connection,
        card_title: &String,
    ) -> Option<(Option<CardId>, Self)> {
        CardDao::select_title(conn, card_title)
            .ok()
            .and_then(|mut cards| cards.pop())
            .map(|card| card.into())
    }
}

#[derive(Clone, Debug)]
#[orm_autobind [(character, place)]]
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
        PlacedCardDao::select_character_and_place(conn, character, place)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to find PlacedCard for {:?} and {:?}: {}",
                    character, place, err
                )
            })
            .into_iter()
            .map(|card| {
                let (id, card) = card.into();
                (id.expect("Selected PlacedCard with no Id"), card)
            })
            .collect::<Vec<_>>()
    }

    pub fn insert_placed_cards(conn: &Connection, cards: Vec<Self>) {
        cards.into_iter().for_each(|card| {
            let card_dao: PlacedCardDao = card.into();
            card_dao.insert(conn).expect("Failed to insert PlacedCard");
        })
    }

    pub fn update_placed_cards(
        conn: &Connection,
        character: &CharacterId,
        place: &FieldPlace,
        update: impl Fn(Vec<Self>) -> Vec<Self>,
    ) -> Vec<Action> {
        let placed_cards = Self::get_placed_cards(conn, character, place);

        let (ids, cards): (Vec<_>, Vec<_>) = placed_cards.into_iter().unzip();
        let cards = update(cards);
        let placed_cards = ids.into_iter().zip(cards).collect::<Vec<_>>();

        placed_cards
            .into_iter()
            .map(|(id, card)| Action {
                sql: PlacedCardDao::update_sql(&["character", "card", "place"], &["id"]),
                params: vec![
                    (
                        ":id".to_string(),
                        serde_json::to_string(&id.0).expect("Failed to serialize Id to Json"),
                    ),
                    (
                        ":character".to_string(),
                        serde_json::to_string(&card.character)
                            .expect("Failed to serialize PlacedCard Character to Json"),
                    ),
                    (
                        ":card".to_string(),
                        serde_json::to_string(&card.card)
                            .expect("Failed to serialize PlacedCard Card to Json"),
                    ),
                    (
                        ":place".to_string(),
                        serde_json::to_string(&card.place)
                            .expect("Failed to serialize PlacedCard Place to Json"),
                    ),
                ],
            })
            .collect()
    }
}

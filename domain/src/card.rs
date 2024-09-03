use rusqlite_orm::orm_bind;

use pipedream_engine::choice::Choice;

#[derive(Clone, Debug)]
#[orm_bind ({name: "$.choice.summary"}, [])]
pub struct Card {
    pub choice: Choice,
}
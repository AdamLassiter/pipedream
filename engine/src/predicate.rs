use std::fmt::Debug;

use rusqlite::{Connection, Result, ToSql};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Predicate {
    pub sql: String,
    pub params: Vec<(String, Value)>,
}

impl Predicate {
    pub fn test(&self, conn: &Connection) -> Result<bool> {
        let params = self
            .params
            .iter()
            .map(|(k, v)| (k.as_str(), v as &dyn ToSql))
            .collect::<Vec<_>>();
        let result = conn
            .prepare(&self.sql)?
            .query_and_then(params.as_slice(), serde_rusqlite::from_row::<i64>)?
            .next()
            .map(|data| data.unwrap())
            .unwrap_or(0);
        Ok(result > 0)
    }
}

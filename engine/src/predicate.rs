use std::fmt::Debug;

use rusqlite::{Connection, Result, ToSql};
use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Predicate {
    pub sql: String,
    pub params: Vec<(String, String)>,
    pub expected: bool,
}

impl Predicate {
    pub fn pure<T>(sql: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            sql: sql.into(),
            params: vec![],
            expected: true,
        }
    }

    pub fn parameterised<T, U, V>(sql: T, params: Vec<(U, Result<V, Error>)>) -> Self
    where
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        Self {
            sql: sql.into(),
            params: params
                .into_iter()
                .map(|(k, v)| (k.into(), v.expect("Failed to serialize param").into()))
                .collect::<Vec<_>>(),
            expected: true,
        }
    }

    pub fn inverse(self) -> Self {
        Self {
            expected: !self.expected,
            ..self
        }
    }

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

        Ok((result > 0) == self.expected)
    }
}

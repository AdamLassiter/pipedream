use std::fmt::Debug;

use rusqlite::{Connection, Result, ToSql};
use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub sql: String,
    pub params: Vec<(String, String)>,
}

impl Action {
    pub fn pure<T>(sql: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            sql: sql.into(),
            params: vec![],
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
        }
    }

    pub fn run(self, conn: &mut Connection) -> Result<()> {
        let params = self
            .params
            .iter()
            .map(|(k, v)| (k.as_str(), v as &dyn ToSql))
            .collect::<Vec<_>>();

        conn.execute(self.sql.as_str(), params.as_slice())?;
        Ok(())
    }
}

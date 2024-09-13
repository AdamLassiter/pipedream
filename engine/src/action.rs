use std::fmt::Debug;

use rusqlite::{Connection, Result, ToSql};
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub sql_batch: Vec<String>,
    pub params: Vec<(String, Value)>,
}

impl Action {
    pub fn pure<T>(sql_batch: Vec<T>) -> Self
    where
        T: Into<String>,
    {
        Self {
            sql_batch: sql_batch.into_iter().map(|x| x.into()).collect(),
            params: vec![],
        }
    }

    pub fn parameterised<T, U>(sql_batch: Vec<T>, params: Vec<(U, Result<Value, Error>)>) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        Self {
            sql_batch: sql_batch.into_iter().map(|x| x.into()).collect(),
            params: params
                .into_iter()
                .map(|(k, v)| (k.into(), v.expect("Failed to serialize param")))
                .collect::<Vec<_>>(),
        }
    }

    pub fn run(self, conn: &mut Connection) -> Result<()> {
        let tx = conn.transaction()?;
        let params = self
            .params
            .iter()
            .map(|(k, v)| (k.as_str(), v as &dyn ToSql))
            .collect::<Vec<_>>();
        self.sql_batch.iter().for_each(|sql| {
            tx.execute(sql.as_str(), params.as_slice())
                .unwrap_or_else(|_| panic!("Failed to execute {}", sql));
        });
        tx.commit()
    }
}

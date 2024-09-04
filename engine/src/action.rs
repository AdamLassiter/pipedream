use std::fmt::Debug;

use rusqlite::{Connection, Result, ToSql};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct Action {
    pub sql_batch: Vec<String>,
    pub params: Vec<(String, Value)>,
}
impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Action")
            .field("sql_batch", &self.sql_batch)
            .field(
                "params",
                &self.params.iter().map(|(k, _)| k).collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl Action {
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

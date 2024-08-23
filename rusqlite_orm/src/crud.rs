use std::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;

pub struct Crud {}
impl Crud {
    pub fn as_orm_method(&self, table_name: &String) -> TokenStream {
        let create_sql = format!(
            "create table {} ( id integer primary key autoincrement, data json )",
            table_name
        );
        let insert_sql = format!("insert into {} (data) values (:data)", table_name);
        let update_sql = format!("update {} set data = (:data) where id = :id", table_name);
        let query_id_sql = format!("select data from {} where id = :id", table_name);
        let delete_id_sql = format!("delete from {} where id = :id", table_name);

        quote! {
            pub fn table_name() -> &'static str {
                #table_name
            }

            pub fn create_table(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute(#create_sql, [])?;
                Ok(())
            }

            pub fn insert(&self, conn: &rusqlite::Connection) -> rusqlite::Result<i64> {
                let id = conn.execute(
                    #insert_sql,
                    rusqlite::named_params! {":data": serde_json::to_value(self).unwrap()},
                )?;
                Ok(conn.last_insert_rowid())
            }

            pub fn update(&self, conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<()> {
                let id = conn.execute(
                    #update_sql,
                    rusqlite::named_params! {":id": id, ":data": serde_json::to_value(self).unwrap()},
                )?;
                Ok(())
            }

            pub fn query(conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<Option<Self>> {
                Ok(conn.prepare(#query_id_sql)?
                    .query_and_then(rusqlite::named_params! {":id": id}, serde_rusqlite::from_row::<String>)?
                    .map(|data| serde_json::from_str::<Self>(data.unwrap().as_str()).unwrap())
                    .next())
            }

            pub fn delete(conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<()> {
                conn.execute(#delete_id_sql, rusqlite::named_params! {":id": id})?;
                Ok(())
            }

            pub fn query_raw(conn: &rusqlite::Connection, raw_sql: &str, params: &[(&str, &dyn rusqlite::ToSql)]) -> rusqlite::Result<Option<Self>> {
                Ok(conn.prepare(raw_sql)?
                    .query_and_then(params, serde_rusqlite::from_row::<String>)?
                    .map(|data| serde_json::from_str::<Self>(data.unwrap().as_str()).unwrap())
                    .next())
            }

            pub fn execute_raw(conn: &rusqlite::Connection, raw_sql: &str, params: &[(&str, &dyn rusqlite::ToSql)]) -> rusqlite::Result<()> {
                conn.execute(raw_sql, params)?;
                Ok(())
            }
        }
    }
}

impl Crud {
    pub fn as_tokenstreams(&self, table_name: &String) -> Vec<TokenStream> {
        vec![self.as_orm_method(table_name)]
    }
}

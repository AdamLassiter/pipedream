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

            pub fn create_sql() -> &'static str {
                #create_sql
            }

            pub fn create_table(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute(Self::create_sql(), [])?;
                Ok(())
            }

            pub fn insert_sql() -> &'static str {
                #insert_sql
            }

            pub fn insert(&self, conn: &rusqlite::Connection) -> rusqlite::Result<i64> {
                Self::execute_raw(
                    conn,
                    Self::insert_sql(),
                    rusqlite::named_params! {":data": serde_json::to_value(self).unwrap()},
                )?;
                Ok(conn.last_insert_rowid())
            }

            pub fn update_sql() -> &'static str {
                #update_sql
            }

            pub fn update(&self, conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<()> {
                Self::execute_raw(
                    conn,
                    Self::update_sql(),
                    rusqlite::named_params! {":id": id, ":data": serde_json::to_value(self).unwrap()},
                )
            }

            pub fn query_sql() -> &'static str {
                #query_id_sql
            }

            pub fn query(conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<Option<Self>> {
                Self::query_raw(conn, Self::query_sql(), rusqlite::named_params! {":id": id})
            }

            pub fn delete_sql() -> &'static str {
                #delete_id_sql
            }

            pub fn delete(conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<()> {
                Self::execute_raw(conn, Self::delete_sql(), rusqlite::named_params! {":id": id})
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

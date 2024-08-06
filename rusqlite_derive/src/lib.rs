extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, ItemStruct, LitStr, Result, Token,
};

struct JsonPath {
    ident: Ident,
    path: LitStr,
}
impl Parse for JsonPath {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let path = input.parse::<LitStr>()?;

        Ok(JsonPath { ident, path })
    }
}
impl JsonPath {
    fn as_sql(&self, table_name: &String) -> String {
        format!(
            "select data from {} where json_extract(data, '{}') like :{}",
            table_name,
            self.path.value(),
            self.ident
        )
    }
}

struct JsonPaths {
    paths: std::vec::Vec<JsonPath>,
}
impl Parse for JsonPaths {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner: Punctuated<JsonPath, Token![,]> = Punctuated::parse_terminated(input)?;

        Ok(JsonPaths {
            paths: inner.into_iter().collect(),
        })
    }
}
impl JsonPaths {
    fn as_quotes(&self, table_name: &String) -> Vec<TokenStream> {
        self.paths.iter().map(|jp| {
            let query_sql = jp.as_sql(table_name);
            let query_by_ident = format_ident!("query_by_{}", jp.ident);
            let ident_key = format!(":{}", jp.ident);
            quote! {
                pub fn #query_by_ident<T>(conn: &rusqlite::Connection, value: T) -> rusqlite::Result<Vec<Self>> where T: Serialize {
                    Ok(conn.prepare(#query_sql)?
                        .query_and_then(rusqlite::named_params! {#ident_key: serde_json::to_value(value).unwrap()}, serde_rusqlite::from_row::<String>)?
                        .map(|data| serde_json::from_str::<Self>(data.unwrap().as_str()).unwrap())
                        .collect::<Vec<Self>>())
                }
            }
        }).collect()
    }
}

#[proc_macro_attribute]
pub fn json_sql(
    bindings: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let bindings = parse_macro_input!(bindings as JsonPaths);
    let ItemStruct { ident, .. } = {
        let item = item.clone();
        parse_macro_input!(item as ItemStruct)
    };

    let item: TokenStream = item.into();

    let table_name = format!("{}s", ident).to_lowercase();
    let create_sql = format!(
        "create table {} ( id int primary key, data json )",
        table_name
    );
    let insert_sql = format!("insert into {} (data) values (:data)", table_name);
    let query_id_sql = format!("select data from {} where id = :id", table_name);
    let queries_by_ident = TokenStream::from_iter(bindings.as_quotes(&table_name));

    let generated = quote! {
        #item

        #[automatically_derived]
        impl #ident {
            pub fn create_table(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute(#create_sql, [])?;
                Ok(())
            }

            pub fn insert(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                let id = conn.execute(
                    #insert_sql,
                    rusqlite::named_params! {":data": serde_json::to_value(self.clone()).unwrap()},
                )?;
                Ok(())
            }

            pub fn query(conn: &rusqlite::Connection, id: i32) -> rusqlite::Result<Vec<Self>> {
                Ok(conn.prepare(#query_id_sql)?
                    .query_and_then(rusqlite::named_params! {":id": id}, serde_rusqlite::from_row::<String>)?
                    .map(|data| serde_json::from_str::<Self>(data.unwrap().as_str()).unwrap())
                    .collect::<Vec<Self>>())
            }

            #queries_by_ident
        }
    }.into();

    println!("{}", generated);
    generated
}

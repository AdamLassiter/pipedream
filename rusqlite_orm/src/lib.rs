mod binding;
mod product;

extern crate rusqlite;
extern crate serde;
extern crate serde_json;
extern crate serde_rusqlite;

extern crate proc_macro;

use binding::Bindings;
use proc_macro2::TokenStream;
use product::Products;
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input, ItemStruct, Token,
};

struct Attributes {
    bindings: Bindings,
    products: Products,
}
impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bindings_input;
        let _ = braced!(bindings_input in input);
        let bindings = bindings_input.parse::<Bindings>()?;

        let products: Products = input
            .parse::<Token![,]>()
            .and_then(|_| {
                let products_input;
                let _ = bracketed!(products_input in input);

                products_input.parse::<Products>()
            })
            .ok()
            .unwrap_or_default();

        Ok(Self { bindings, products })
    }
}

#[proc_macro_attribute]
pub fn orm_bind(
    attributes: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attributes = parse_macro_input!(attributes as Attributes);
    let ItemStruct { ident, .. } = {
        let item = item.clone();
        parse_macro_input!(item as ItemStruct)
    };

    let item: TokenStream = item.into();

    let table_name = format!("{}s", ident).to_lowercase();
    let create_sql = format!(
        "create table {} ( id integer primary key autoincrement, data json )",
        table_name
    );
    let insert_sql = format!("insert into {} (data) values (:data)", table_name);
    let query_id_sql = format!("select data from {} where id = :id", table_name);

    let bindings_queries = TokenStream::from_iter(attributes.bindings.as_tokenstreams(&table_name));
    let products_queries = TokenStream::from_iter(
        attributes
            .products
            .as_tokenstreams(&table_name, &attributes.bindings),
    );

    quote! {
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

            pub fn query_by_id(conn: &rusqlite::Connection, id: i32) -> rusqlite::Result<Option<Self>> {
                Ok(conn.prepare(#query_id_sql)?
                    .query_and_then(rusqlite::named_params! {":id": id}, serde_rusqlite::from_row::<String>)?
                    .map(|data| serde_json::from_str::<Self>(data.unwrap().as_str()).unwrap())
                    .next())
            }

            pub fn query_raw(conn: &rusqlite::Connection, raw_sql: &str, params: &[(&str, &dyn rusqlite::ToSql)]) -> rusqlite::Result<Option<Self>> {
                Ok(conn.prepare(raw_sql)?
                    .query_and_then(params, serde_rusqlite::from_row::<String>)?
                    .map(|data| serde_json::from_str::<Self>(data.unwrap().as_str()).unwrap())
                    .next())
            }

            #bindings_queries

            #products_queries
        }
    }.into()
}

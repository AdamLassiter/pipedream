use std::{collections::BTreeMap, vec::Vec};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Result, Token,
};

pub struct Binding {
    ident: Ident,
    path: LitStr,
}
impl Parse for Binding {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let path = input.parse::<LitStr>()?;

        Ok(Binding { ident, path })
    }
}
impl Binding {
    fn as_query_sql(&self, table_name: &String) -> String {
        format!(
            "select id, data from {} where data->'{}' = :{}",
            table_name,
            self.path.value(),
            self.ident
        )
    }

    fn as_update_sql(&self, table_name: &String) -> String {
        format!(
            "update {} set data = json_replace(data, '{}', json(:{})) where id = :id",
            table_name,
            self.path.value(),
            self.ident
        )
    }

    pub fn as_orm_method(&self, table_name: &String) -> TokenStream {
        let query_sql = self.as_query_sql(table_name);
        let update_sql = self.as_update_sql(table_name);
        let query_by_ident = format_ident!("query_by_{}", self.ident);
        let update_ident = format_ident!("update_{}", self.ident);
        let ident_key = format!(":{}", self.ident);

        quote! {
            pub fn #query_by_ident<T>(conn: &rusqlite::Connection, value: &T) -> rusqlite::Result<std::vec::Vec<(i64, Self)>> where T: serde::Serialize {
                Ok(conn.prepare(#query_sql)?
                    .query_and_then(rusqlite::named_params! {#ident_key: serde_json::to_value(value).unwrap()}, serde_rusqlite::from_row::<(i64, String)>)?
                    .map(|res| {
                        let (id, data) = res.unwrap();
                        (id, serde_json::from_str::<Self>(data.as_str()).unwrap())
                    })
                    .collect::<std::vec::Vec<(i64, Self)>>())
            }

            pub fn #update_ident<T>(conn: &rusqlite::Connection, id: i64, value: &T) -> rusqlite::Result<()> where T: serde::Serialize {
                conn.prepare(#update_sql)?
                    .execute(rusqlite::named_params! {":id": id, #ident_key: serde_json::to_value(value).unwrap()})?;
                Ok(())
            }
        }
    }
}

pub struct Bindings {
    bindings: Vec<Binding>,
}
impl Parse for Bindings {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner: Punctuated<Binding, Token![,]> = Punctuated::parse_terminated(input)?;

        Ok(Bindings {
            bindings: inner.into_iter().collect(),
        })
    }
}
impl Bindings {
    pub fn as_tokenstreams(&self, table_name: &String) -> Vec<TokenStream> {
        self.bindings
            .iter()
            .map(|bind| bind.as_orm_method(table_name))
            .collect()
    }

    pub fn as_map(&self) -> BTreeMap<&Ident, &LitStr> {
        self.bindings
            .iter()
            .map(|bind| (&bind.ident, &bind.path))
            .collect::<BTreeMap<_, _>>()
    }
}

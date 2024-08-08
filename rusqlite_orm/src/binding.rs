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
    pub fn as_sql(&self, table_name: &String) -> String {
        format!(
            "select data from {} where data->'{}' = :{}",
            table_name,
            self.path.value(),
            self.ident
        )
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
        self.bindings.iter().map(|bind| {
            let query_sql = bind.as_sql(table_name);
            let query_by_ident = format_ident!("query_by_{}", bind.ident);
            let ident_key = format!(":{}", bind.ident);
            quote! {
                pub fn #query_by_ident<T>(conn: &rusqlite::Connection, value: &T) -> rusqlite::Result<std::vec::Vec<Self>> where T: serde::Serialize {
                    Ok(conn.prepare(#query_sql)?
                        .query_and_then(rusqlite::named_params! {#ident_key: serde_json::to_value(value).unwrap()}, serde_rusqlite::from_row::<String>)?
                        .map(|data| serde_json::from_str::<Self>(data.unwrap().as_str()).unwrap())
                        .collect::<std::vec::Vec<Self>>())
                }
            }
        }).collect()
    }

    pub fn as_map(&self) -> BTreeMap<&Ident, &LitStr> {
        self.bindings
            .iter()
            .map(|bind| (&bind.ident, &bind.path))
            .collect::<BTreeMap<_, _>>()
    }
}

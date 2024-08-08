use std::{collections::BTreeMap, vec::Vec};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Result, Token,
};

use crate::binding::Bindings;

pub struct Product {
    pub idents: Vec<Ident>,
}
impl Parse for Product {
    fn parse(input: ParseStream) -> Result<Self> {
        let product_input;
        let _ = parenthesized!(product_input in input);
        let idents: Punctuated<Ident, Token![,]> = Punctuated::parse_terminated(&product_input)?;
        let idents = idents.into_iter().collect::<Vec<_>>();

        Ok(Product { idents })
    }
}
impl Product {
    pub fn as_sql(&self, table_name: &String, bindings: &BTreeMap<&Ident, &LitStr>) -> String {
        let bindings = self
            .idents
            .iter()
            .map(|ident| {
                format!(
                    "data->'{}' = :{}",
                    bindings
                        .get(ident)
                        .expect("Unrecognised Ident in Product")
                        .value(),
                    ident
                )
            })
            .collect::<Vec<_>>();

        format!(
            "select data from {} where {}",
            table_name,
            bindings.join(" and ")
        )
    }

    pub fn as_query_by_ident(&self) -> Ident {
        let idents = self
            .idents
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<_>>();

        format_ident!("query_by_{}", idents.join("_and_"))
    }
}

#[derive(Default)]
pub struct Products {
    products: Vec<Product>,
}
impl Parse for Products {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner = Punctuated::<Product, Token![,]>::parse_terminated(input)?;

        Ok(Products {
            products: inner.into_iter().collect(),
        })
    }
}
impl Products {
    pub fn as_tokenstreams(&self, table_name: &String, bindings: &Bindings) -> Vec<TokenStream> {
        self.products.iter().map(|product| {
            let bindings = bindings.as_map();
            let query_sql = product.as_sql(table_name, &bindings);
            let query_by_ident = product.as_query_by_ident();
            let named_params = product.idents.iter()
                .map(|ident|{
                    let bind = format!(":{}", ident);
                    quote! { #bind: serde_json::to_value(#ident).unwrap() }
                })
                .collect::<Vec<_>>();
            let generics = (0..named_params.len()).map(|i| {
                let vartype = format_ident!("T{}", i);
                quote! { #vartype }
            }).collect::<Vec<_>>();
            let type_bounds = generics.iter().map(|vartype| {
                quote! {#vartype: Serialize}
            }).collect::<Vec<_>>();
            let arguments = product.idents.iter().enumerate().map(|(i, ident)| {
                let vartype = format_ident!("T{}", i);
                quote! { #ident: &#vartype }
            }).collect::<Vec<_>>();

            quote! {
                pub fn #query_by_ident<#(#generics),*>(conn: &rusqlite::Connection, #(#arguments),*) -> rusqlite::Result<std::vec::Vec<Self>> where #(#type_bounds),* {
                    Ok(conn.prepare(#query_sql)?
                        .query_and_then(rusqlite::named_params! {#(#named_params),*}, serde_rusqlite::from_row::<String>)?
                        .map(|data| serde_json::from_str::<Self>(data.unwrap().as_str()).unwrap())
                        .collect::<Vec<Self>>())
                }
            }
        }).collect()
    }
}

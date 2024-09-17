use std::collections::BTreeMap;
use std::vec::Vec;

use crate::column::Column;
use proc_macro::{Diagnostic, Level, Span};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Result, Token,
};

#[derive(Debug)]
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
    fn as_sql(&self, table_name: &String) -> String {
        format!(
            "select * from {} where {}",
            table_name,
            self.idents
                .iter()
                .map(|ident| format!("{} = :{}", ident, ident))
                .collect::<Vec<_>>()
                .join(" and ")
        )
    }

    fn as_select_ident(&self) -> Ident {
        let idents = self
            .idents
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<_>>();

        format_ident!("select_{}", idents.join("_and_"))
    }

    pub fn as_orm_method(
        &self,
        table_name: &String,
        bindings: &[Column],
        columns: &TokenStream,
    ) -> TokenStream {
        let query_sql = self.as_sql(table_name);
        let select_ident = self.as_select_ident();
        let column_types = bindings
            .iter()
            .map(|col| (col.ident.clone(), col))
            .collect::<BTreeMap<_, _>>();
        let column_args = self
            .idents
            .iter()
            .map(|ident| {
                column_types.get(ident).unwrap_or_else(|| {
                    Diagnostic::spanned(
                        vec![Span::call_site(), ident.span().unwrap()],
                        Level::Error,
                        format!("Identifier {:?} was not bound to a Dao Column", ident),
                    )
                    .emit();
                    panic!("Identifier was not bound to a Dao Column")
                })
            })
            .collect::<Vec<_>>();
        let arguments = column_args
            .iter()
            .map(|col| col.as_method_arg())
            .collect::<Vec<_>>();
        let serde_exprs = column_args
            .iter()
            .map(|col| col.as_serde_value())
            .collect::<Vec<_>>();

        quote! {
            pub fn #select_ident(conn: &rusqlite::Connection, #(#arguments),*) -> rusqlite::Result<std::vec::Vec<Self>> {
                Ok(conn.prepare(#query_sql)?
                    .query_and_then(rusqlite::named_params! {#(#serde_exprs),*}, |row| serde_rusqlite::from_row_with_columns::<Self>(row, #columns))?
                    .map(|row| row.expect("Sql for Product was not valid Json"))
                    .collect::<Vec<Self>>())
            }
        }
    }
}

#[derive(Default, Debug)]
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
    pub fn as_orm_methods(
        &self,
        table_name: &String,
        bindings: &[Column],
        columns: &TokenStream,
    ) -> TokenStream {
        TokenStream::from_iter(
            self.products
                .iter()
                .map(|product| product.as_orm_method(table_name, bindings, columns)),
        )
    }
}

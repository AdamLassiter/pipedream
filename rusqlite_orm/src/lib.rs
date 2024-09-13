#![feature(iter_intersperse)]

mod binding;
mod crud;
mod dao;
mod product;

extern crate proc_macro;

use binding::Bindings;
use crud::Crud;
use dao::Dao;
use proc_macro2::TokenStream;
use product::Products;
use quote::{format_ident, quote};
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input, ItemStruct, Token,
};

struct Attributes {
    crud: Crud,
    bindings: Bindings,
    products: Products,
}
impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let crud = Crud {};

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

        Ok(Self {
            crud,
            bindings,
            products,
        })
    }
}

#[proc_macro_attribute]
pub fn orm_bind(
    attributes: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attributes = parse_macro_input!(attributes as Attributes);
    let item_struct = {
        let item = item.clone();
        parse_macro_input!(item as ItemStruct)
    };
    let ident = item_struct.ident;

    let item: TokenStream = item.into();

    let ident_id = format_ident!("{}Id", ident);
    let table_name = format!("{}s", ident).to_lowercase();
    let crud_queries =
        TokenStream::from_iter(attributes.crud.as_tokenstreams(&ident_id, &table_name));
    let bindings_queries =
        TokenStream::from_iter(attributes.bindings.as_tokenstreams(&ident_id, &table_name));
    let products_queries = TokenStream::from_iter(attributes.products.as_tokenstreams(
        &ident_id,
        &table_name,
        &attributes.bindings,
    ));

    let code = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #item

        #[derive(Clone, Copy, PartialEq, Eq, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
        pub struct #ident_id(pub i64);

        #[automatically_derived]
        impl #ident {
            #crud_queries
        }


        #[automatically_derived]
        impl #ident {
            #bindings_queries
        }

        #[automatically_derived]
        impl #ident {
            #products_queries
        }
    };
    // println!("{}", code);
    code.into()
}

#[proc_macro_attribute]
pub fn orm_autobind(
    _attributes: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_struct = {
        let item = item.clone();
        parse_macro_input!(item as ItemStruct)
    };
    let item: TokenStream = item.into();

    let ident_id = format_ident!("{}Id", item_struct.ident);

    let dao = Dao::from(item_struct);
    let dao_defs = dao.as_tokenstream();

    let code = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #item

        #[derive(Clone, Copy, PartialEq, Eq, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
        pub struct #ident_id(pub i64);

        #dao_defs
    };
    println!("{}", code);
    code.into()
}

mod binding;
mod crud;
mod product;

extern crate rusqlite;
extern crate serde;
extern crate serde_json;
extern crate serde_rusqlite;

extern crate proc_macro;

use binding::Bindings;
use crud::Crud;
use proc_macro2::TokenStream;
use product::Products;
use quote::quote;
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
    let ItemStruct { ident, .. } = {
        let item = item.clone();
        parse_macro_input!(item as ItemStruct)
    };

    let item: TokenStream = item.into();

    let table_name = format!("{}s", ident).to_lowercase();
    let crud_queries = TokenStream::from_iter(attributes.crud.as_tokenstreams(&table_name));
    let bindings_queries = TokenStream::from_iter(attributes.bindings.as_tokenstreams(&table_name));
    let products_queries = TokenStream::from_iter(
        attributes
            .products
            .as_tokenstreams(&table_name, &attributes.bindings),
    );

    quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #item

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
    }
    .into()
}

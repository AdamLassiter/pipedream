#![feature(iter_intersperse)]

mod column;
mod dao;
mod sql_type;

extern crate proc_macro;

use dao::Dao;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, ItemStruct,
};

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
    // println!("{}", code);
    code.into()
}

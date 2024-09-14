#![feature(iter_intersperse)]

mod column;
mod dao;
mod sql_type;
mod product;

extern crate proc_macro;

use dao::Dao;
use proc_macro2::TokenStream;
use product::Products;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, ItemStruct,
};

#[proc_macro_attribute]
pub fn orm_autobind(
    attributes: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attributes = parse_macro_input!(attributes as Products);
    let item_struct = {
        let item = item.clone();
        parse_macro_input!(item as ItemStruct)
    };
    let item: TokenStream = item.into();

    let ident_id = format_ident!("{}Id", item_struct.ident);
    let table_name = format!("{}s", item_struct.ident).to_lowercase();

    let dao = Dao::from((attributes, item_struct));
    let dao_defs = dao.as_orm_methods(&ident_id, &table_name);

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

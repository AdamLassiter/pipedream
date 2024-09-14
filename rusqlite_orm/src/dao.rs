use std::str::FromStr;
use std::vec::Vec;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, Ident, ItemStruct};

use crate::column::Column;
use crate::product::Products;
use crate::sql_type::Type;

pub struct Dao {
    ident: Ident,
    columns: Vec<Column>,
    products: Products,
}
impl Dao {
    fn as_table_defn(&self) -> String {
        self.columns
            .iter()
            .map(|col| col.as_table_defn())
            .intersperse(", ".into())
            .collect::<String>()
    }

    fn as_row_defn(&self) -> String {
        self.columns
            .iter()
            .map(|col| format!(":{}", col.ident))
            .intersperse(", ".into())
            .collect::<String>()
    }

    fn as_columns(&self) -> TokenStream {
        let arguments = self
            .columns
            .iter()
            .map(|col| {
                let col_name = format!("\"{}\".to_string()", col.ident);
                TokenStream::from_str(col_name.as_str()).unwrap()
            })
            .collect::<Vec<_>>();
        quote! { &[ #(#arguments),* ] }
    }

    fn as_values(&self) -> TokenStream {
        let arguments = self
            .columns
            .iter()
            .map(Column::as_value)
            .collect::<Vec<_>>();
        quote! { #(#arguments),* }
    }

    fn create(&self, table_name: &String) -> TokenStream {
        let table_defn = self.as_table_defn();
        let create_sql = format!("create table {} ({})", table_name, table_defn);

        quote! {
            pub fn create_table(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute(#create_sql, ())?;
                Ok(())
            }
        }
    }

    fn insert(&self, ident_id: &Ident, table_name: &String) -> TokenStream {
        let row_defn = self.as_row_defn();
        let update_sql = format!("insert into {} values ({})", table_name, row_defn);
        let dao_fieldnames = TokenStream::from_iter(
            self.columns
                .iter()
                .map(|Column { ident, .. }| quote! { #ident, }),
        );
        let values = self.as_values();

        quote! {
            pub fn insert(self, conn: &rusqlite::Connection) -> rusqlite::Result<#ident_id> {
                let Self {
                    #dao_fieldnames
                } = self;
                conn.prepare(#update_sql)?
                    .execute(rusqlite::named_params! {#values})?;
                Ok(#ident_id(conn.last_insert_rowid()))
            }
        }
    }

    pub fn as_orm_methods(&self, ident_id: &Ident, table_name: &String) -> TokenStream {
        let Self { ident, columns, products } = self;
        let ident_dao = format_ident!("{}Dao", ident);

        let create = self.create(&table_name);
        let insert = self.insert(&ident_id, &table_name);
        let fields = TokenStream::from_iter(columns.iter().map(Column::as_struct_defn));
        let methods = TokenStream::from_iter(
            columns
                .iter()
                .map(|col| col.as_orm_methods(&ident_id, &table_name, &self.as_columns())),
        );
        let product_methods = products.as_orm_methods(&table_name, &self.columns);
        let ident_fieldnames = TokenStream::from_iter(
            columns
                .iter()
                .filter(|col| col.typ != Type::PrimaryKey)
                .map(|Column { ident, .. }| quote! { #ident, }),
        );
        let dao_fieldnames = TokenStream::from_iter(
            columns
                .iter()
                .map(|Column { ident, .. }| quote! { #ident, }),
        );
        let dao_values = TokenStream::from_iter(
            columns
                .iter()
                .filter(|col| col.typ != Type::PrimaryKey)
                .map(Column::as_into_value),
        );
        let ident_values = TokenStream::from_iter(
            columns
                .iter()
                .filter(|col| col.typ != Type::PrimaryKey)
                .map(Column::as_from_value),
        );

        quote! {
            #[derive(Clone, PartialEq, Eq, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
            pub struct #ident_dao {
                #fields
            }
            #[automatically_derived]
            impl #ident_dao {
                #create
                #insert
                #methods
                #product_methods
            }
            #[automatically_derived]
            impl From<#ident> for #ident_dao {
                fn from(other: #ident) -> Self {
                    let #ident {
                        #ident_fieldnames
                    } = other;
                    Self {
                        id: None,
                        #dao_values
                    }
                }
            }
            #[automatically_derived]
            impl From<(Option<#ident_id>, #ident)> for #ident_dao {
                fn from((id, other): (Option<#ident_id>, #ident)) -> Self {
                    let #ident {
                        #ident_fieldnames
                    } = other;
                    Self {
                        id: id.map(|i| i.0),
                        #dao_values
                    }
                }
            }
            #[automatically_derived]
            impl Into<(Option<#ident_id>, #ident)> for  #ident_dao {
                fn into(self) -> (Option<#ident_id>, #ident) {
                    let Self {
                        #dao_fieldnames
                    } = self;
                    (id.map(|i| #ident_id(i)), #ident {
                        #ident_values
                    })
                }
            }
            #[automatically_derived]
            impl Into<#ident> for  #ident_dao {
                fn into(self) -> #ident {
                    let Self {
                        #dao_fieldnames
                    } = self;
                    #ident {
                        #ident_values
                    }
                }
            }
        }
    }
}
impl From<(Products, ItemStruct)> for Dao {
    fn from((products, value): (Products, ItemStruct)) -> Self {
        let ident = value.ident;
        let mut cols_no_id = match value.fields {
            Fields::Named(fs) => fs,
            _ => panic!("Not a NamedFields struct"),
        }
        .named
        .into_iter()
        .map(|field| field.into())
        .collect::<Vec<Column>>();

        let mut columns = vec![Column {
            ident: format_ident!("id"),
            typ: Type::PrimaryKey,
        }];
        columns.append(&mut cols_no_id);

        Self {
            ident,
            products,
            columns,
        }
    }
}

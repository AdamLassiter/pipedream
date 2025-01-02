use std::str::FromStr;
use std::vec::Vec;

use proc_macro::{Diagnostic, Level, Span};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, Ident, ItemStruct};

use crate::column::Column;
use crate::product::Products;
use crate::sql_type::Type;

pub struct Dao {
    ident: Ident,
    bindings: Vec<Column>,
    products: Products,
}
impl Dao {
    fn as_table_defn(&self) -> String {
        self.bindings
            .iter()
            .map(|col| col.as_table_defn())
            .intersperse(", ".into())
            .collect::<String>()
    }

    fn as_row_defn(&self) -> String {
        self.bindings
            .iter()
            .map(|col| format!(":{}", col.ident))
            .intersperse(", ".into())
            .collect::<String>()
    }

    fn as_columns(&self) -> TokenStream {
        let arguments = self
            .bindings
            .iter()
            .map(|col| {
                let col_name = format!("\"{}\".to_string()", col.ident);
                TokenStream::from_str(col_name.as_str())
                    .expect("Could not prepare static Column definition")
            })
            .collect::<Vec<_>>();
        quote! { &[ #(#arguments),* ] }
    }

    fn as_values(&self) -> TokenStream {
        let arguments = self
            .bindings
            .iter()
            .map(Column::as_value)
            .collect::<Vec<_>>();
        quote! { #(#arguments),* }
    }

    fn table_name(&self, table_name: &String) -> TokenStream {
        quote! {
            pub fn table_name() -> &'static str {
                #table_name
            }
        }
    }

    fn create_sql(&self, table_name: &String) -> TokenStream {
        let table_defn = self.as_table_defn();
        let create_sql = format!("create table if not exists {} ({})", table_name, table_defn);

        quote! {
            pub fn create_table_sql() -> String {
                #create_sql.into()
            }
        }
    }

    fn create(&self) -> TokenStream {
        quote! {
            pub fn create_table(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute(&Self::create_table_sql(), ())?;
                Ok(())
            }
        }
    }

    fn drop_sql(&self, table_name: &String) -> TokenStream {
        let drop_sql = format!("drop table if exists {}", table_name);

        quote! {
            pub fn drop_table_sql() -> String {
                #drop_sql.into()
            }
        }
    }

    fn drop(&self) -> TokenStream {
        quote! {
            pub fn drop_table(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute(&Self::drop_table_sql(), ())?;
                Ok(())
            }
        }
    }

    fn insert(&self, ident_id: &Ident, table_name: &String) -> TokenStream {
        let row_defn = self.as_row_defn();
        let insert_sql = format!("insert into {} values ({})", table_name, row_defn);
        let dao_fieldnames = TokenStream::from_iter(
            self.bindings
                .iter()
                .map(|Column { ident, .. }| quote! { #ident, }),
        );
        let values = self.as_values();

        quote! {
            pub fn insert(self, conn: &rusqlite::Connection) -> rusqlite::Result<#ident_id> {
                let Self {
                    #dao_fieldnames
                } = self;
                conn.prepare(#insert_sql)?
                    .execute(rusqlite::named_params! {#values})?;
                Ok(#ident_id(conn.last_insert_rowid()))
            }
        }
    }

    fn select_sql(&self, table_name: &String) -> TokenStream {
        quote! {
            pub fn select_sql(select_cols: &[&str], where_cols: &[&str]) -> String {
                let select_clause = select_cols.iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<_>>()
                    .join(", ");
                let where_clause = where_cols.iter()
                    .map(|s| format!("{} = :{}", s, s))
                    .collect::<Vec<_>>()
                    .join(" and ");
                format!("select {} from {} where {}", select_clause, #table_name, where_clause)
            }
        }
    }

    fn update_sql(&self, table_name: &String) -> TokenStream {
        quote! {
            pub fn update_sql(set_cols: &[&str], where_cols: &[&str]) -> String {
                let set_clause = set_cols.iter()
                    .map(|s| format!("{} = :{}", s, s))
                    .collect::<Vec<_>>()
                    .join(", ");
                let where_clause = where_cols.iter()
                    .map(|s| format!("{} = :{}", s, s))
                    .collect::<Vec<_>>()
                    .join(" and ");
                format!("update {} set {} where {}", #table_name, set_clause, where_clause)
            }
        }
    }

    pub fn as_orm_methods(&self, ident_id: &Ident, table_name: &String) -> TokenStream {
        let Self {
            ident,
            bindings,
            products,
        } = self;
        let ident_dao = format_ident!("{}Dao", ident);

        let table_name_fn = self.table_name(table_name);
        let create_sql_fn = self.create_sql(table_name);
        let create_fn = self.create();
        let drop_sql_fn = self.drop_sql(table_name);
        let drop_fn = self.drop();
        let insert_fn = self.insert(ident_id, table_name);
        let select_sql_fn = self.select_sql(table_name);
        let update_sql_fn = self.update_sql(table_name);
        let fields = TokenStream::from_iter(bindings.iter().map(Column::as_struct_defn));
        let columns = self.as_columns();
        let column_fns = TokenStream::from_iter(
            bindings
                .iter()
                .map(|col| col.as_orm_methods(ident_id, &columns)),
        );
        let product_fns = products.as_orm_methods(ident_id, &self.bindings, &columns);
        let ident_fieldnames = TokenStream::from_iter(
            bindings
                .iter()
                .filter(|col| !matches!(col.typ, Type::PrimaryKey))
                .map(|Column { ident, .. }| quote! { #ident, }),
        );
        let dao_fieldnames = TokenStream::from_iter(
            bindings
                .iter()
                .map(|Column { ident, .. }| quote! { #ident, }),
        );
        let dao_values = TokenStream::from_iter(
            bindings
                .iter()
                .filter(|col| !matches!(col.typ, Type::PrimaryKey))
                .map(Column::as_into_value),
        );
        let ident_values = TokenStream::from_iter(
            bindings
                .iter()
                .filter(|col| !matches!(col.typ, Type::PrimaryKey))
                .map(Column::as_from_value),
        );

        quote! {
            #[derive(Clone, PartialEq, Eq, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
            pub struct #ident_dao {
                #fields
            }
            #[automatically_derived]
            impl #ident_dao {
                #table_name_fn

                #create_sql_fn
                #create_fn

                #drop_sql_fn
                #drop_fn

                #insert_fn

                #select_sql_fn
                #update_sql_fn

                #column_fns
                #product_fns
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
        let ident = value.ident.clone();
        let mut cols_no_id = match value.fields {
            Fields::Named(fs) => fs,
            _ => {
                Diagnostic::spanned(
                    vec![Span::call_site(), value.ident.span().unwrap()],
                    Level::Error,
                    format!("Fields of {:?} not a NamedFields struct", value.ident),
                )
                .emit();
                panic!("Fields not a NamedFields struct")
            }
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
            bindings: columns,
        }
    }
}

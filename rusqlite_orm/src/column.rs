use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident};

use crate::sql_type::Type;

pub struct Column {
    pub ident: Ident,
    pub typ: Type,
}
impl Column {
    fn ident_key(&self) -> String {
        format!(":{}", self.ident)
    }

    pub fn as_table_defn(&self) -> String {
        format!("{} {}", self.ident, self.typ.as_table_defn())
    }

    pub fn as_value(&self) -> TokenStream {
        let ident = &self.ident;
        let ident_key = self.ident_key();
        quote! { #ident_key: #ident }
    }

    pub fn as_serde_value(&self) -> TokenStream {
        let ident = &self.ident;
        let ident_key = self.ident_key();
        match self.typ {
            Type::Json(_) => quote! { #ident_key: serde_json::to_value(#ident).expect("Failed to serialize Column to Json Value") },
            _ => quote! { #ident_key: #ident },
        }
    }

    pub fn as_method_arg(&self) -> TokenStream {
        let ident = &self.ident;
        let typ = self.typ.as_method_arg();
        quote! { #ident: &#typ }
    }

    fn select(&self, table_name: &String, columns: &TokenStream) -> TokenStream {
        let method_arg = self.as_method_arg();
        let select_ident = format_ident!("select_{}", self.ident);
        let select_sql = format!(
            "select * from {} where {} = {}",
            table_name, self.ident, self.ident_key()
        );
        let serde_expr = self.as_serde_value();

        quote! {
            pub fn #select_ident(conn: &rusqlite::Connection, #method_arg) -> rusqlite::Result<std::vec::Vec<Self>> {
                Ok(conn.prepare(#select_sql)?
                    .query_and_then(rusqlite::named_params! {#serde_expr}, |row| serde_rusqlite::from_row_with_columns::<Self>(row, #columns))?
                    .map(|row| row.expect("Sql for Column was not valid Json"))
                    .collect::<std::vec::Vec<Self>>())
            }
        }
    }

    fn count(&self, table_name: &String) -> TokenStream {
        let method_arg = self.as_method_arg();
        let count_ident = format_ident!("count_{}", self.ident);
        let count_sql = format!(
            "select count(*) from {} where {} = {}",
            table_name, self.ident, self.ident_key()
        );
        let serde_expr = self.as_serde_value();

        quote! {
            pub fn #count_ident(conn: &rusqlite::Connection, #method_arg) -> rusqlite::Result<i64> {
                Ok(conn.prepare(#count_sql)?
                    .query_and_then(rusqlite::named_params! {#serde_expr}, |row| serde_rusqlite::from_row_with_columns::<i64>(row, &["count(*)".to_string()]))?
                    .map(|row| row.expect("Failed to deserialize Column from Sql"))
                    .next().unwrap_or(0))
            }
        }
    }

    fn update(&self, ident_id: &Ident, table_name: &String) -> TokenStream {
        let method_arg = self.as_method_arg();
        let update_ident = format_ident!("update_{}", self.ident);
        let update_sql = format!(
            "update {} set {} = {} where id = :id",
            table_name, self.ident, self.ident_key()
        );
        let serde_expr = self.as_serde_value();

        quote! {
            pub fn #update_ident(conn: &rusqlite::Connection, id: &#ident_id, #method_arg) -> rusqlite::Result<()> {
                conn.prepare(#update_sql)?
                    .execute(rusqlite::named_params! {":id": id.0, #serde_expr})?;
                Ok(())
            }
        }
    }

    pub fn as_orm_methods(&self, ident_id: &Ident, table_name: &String, columns: &TokenStream) -> TokenStream {
        let select = self.select(table_name, columns);
        let count = if self.typ != Type::PrimaryKey {
            self.count(table_name)
        } else {
            quote! {}
        };
        let update = if self.typ != Type::PrimaryKey {
            self.update(ident_id, table_name)
        } else {
            quote! {}
        };

        quote! {
            #select
            #count
            #update
        }
    }

    pub fn as_struct_defn(&self) -> TokenStream {
        let Self { ident, typ } = self;
        let typ = typ.as_tosql();

        quote! {
            #ident: #typ,
        }
    }

    pub fn as_into_value(&self) -> TokenStream {
        let Self { ident, typ } = self;
        match typ {
            Type::PrimaryKey => quote! { #ident: None, },
            Type::Null(_typ) => quote! { (), },
            Type::Integer(_typ) | Type::Real(_typ) | Type::Text(_typ) => quote! { #ident: #ident, },
            Type::Json(_typ) => quote! {
                #ident: serde_json::to_value(#ident).expect("Could not serialize into Dao Value"),
            },
        }
    }

    pub fn as_from_value(&self) -> TokenStream {
        let Self { ident, typ } = self;
        match typ {
            Type::PrimaryKey => quote! {},
            Type::Null(_typ) => quote! { #ident: (), },
            Type::Integer(_typ) | Type::Real(_typ) | Type::Text(_typ) => quote! { #ident: #ident, },
            Type::Json(_typ) => quote! {
                #ident: serde_json::from_str(#ident.as_str().expect("Dao Value was not a String"))
                        .expect("Dao Value was not valid Json"),
            },
        }
    }
}
impl From<Field> for Column {
    fn from(field: Field) -> Self {
        Self {
            ident: field.ident.expect("No Ident on Field in struct"),
            typ: field.ty.into(),
        }
    }
}

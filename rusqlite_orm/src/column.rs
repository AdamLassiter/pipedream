use proc_macro::{Diagnostic, Level, Span};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident, spanned::Spanned};

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
        quote! { #ident_key: *#ident }
    }

    pub fn as_serde_value(&self) -> TokenStream {
        let ident = &self.ident;
        let ident_key = self.ident_key();
        match self.typ {
            Type::Json(_) => {
                quote! { #ident_key: serde_json::to_value(#ident).expect("Failed to serialize Column to Json Value") }
            }
            Type::PrimaryKey => quote! { #ident_key: #ident.0 },
            _ => quote! { #ident_key: #ident },
        }
    }

    pub fn as_method_arg(&self, ident_id: &Ident) -> TokenStream {
        let Self { ident, typ } = self;
        let typ = typ.as_method_arg();
        match self.typ {
            Type::PrimaryKey => quote! { #ident: &#ident_id },
            _ => quote! { #ident: &#typ },
        }
    }

    fn select(&self, ident_id: &Ident, columns: &TokenStream) -> TokenStream {
        let method_arg = self.as_method_arg(ident_id);
        let select_ident = format_ident!("select_{}", self.ident);
        let ident = self.ident.to_string();
        let serde_expr = self.as_serde_value();

        quote! {
            pub fn #select_ident(conn: &rusqlite::Connection, #method_arg) -> rusqlite::Result<std::vec::Vec<Self>> {
                let select_sql = Self::select_sql(&["*"], &[#ident]);
                Ok(conn.prepare(&select_sql)?
                    .query_and_then(rusqlite::named_params! {#serde_expr}, |row| serde_rusqlite::from_row_with_columns::<Self>(row, #columns))?
                    .map(|row| row.expect("Sql for Column was not valid Json"))
                    .collect::<std::vec::Vec<Self>>())
            }
        }
    }

    fn count(&self, ident_id: &Ident) -> TokenStream {
        if matches!(self.typ, Type::PrimaryKey) {
            return quote! {};
        }
        let method_arg = self.as_method_arg(ident_id);
        let count_ident = format_ident!("count_{}", self.ident);
        let ident = self.ident.to_string();
        let serde_expr = self.as_serde_value();

        quote! {
            pub fn #count_ident(conn: &rusqlite::Connection, #method_arg) -> rusqlite::Result<i64> {
                let count_sql = Self::select_sql(&["count(*)"], &[#ident]);
                Ok(conn.prepare(&count_sql)?
                    .query_and_then(rusqlite::named_params! {#serde_expr}, |row| serde_rusqlite::from_row_with_columns::<i64>(row, &["count(*)".to_string()]))?
                    .map(|row| row.expect("Failed to deserialize Column from Sql"))
                    .next().unwrap_or(0))
            }
        }
    }

    fn update(&self, ident_id: &Ident) -> TokenStream {
        if matches!(self.typ, Type::PrimaryKey) {
            return quote! {};
        }
        let method_arg = self.as_method_arg(ident_id);
        let update_ident = format_ident!("update_{}", self.ident);
        let ident = self.ident.to_string();
        let serde_expr = self.as_serde_value();

        quote! {
            pub fn #update_ident(conn: &rusqlite::Connection, id: &#ident_id, #method_arg) -> rusqlite::Result<()> {
                let update_sql = Self::update_sql(&[#ident], &["id"]);
                conn.prepare(&update_sql)?
                    .execute(rusqlite::named_params! {":id": id.0, #serde_expr})?;
                Ok(())
            }
        }
    }

    pub fn as_orm_methods(&self, ident_id: &Ident, columns: &TokenStream) -> TokenStream {
        let select = self.select(ident_id, columns);
        let count = self.count(ident_id);
        let update = self.update(ident_id);

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
            Type::Json(_typ) => {
                let serde_err = format!("Could not serialize {ident} into Dao Value");
                quote! {
                    #ident: serde_json::to_value(#ident).expect(#serde_err),
                }
            }
        }
    }

    pub fn as_from_value(&self) -> TokenStream {
        let Self { ident, typ } = self;
        match typ {
            Type::PrimaryKey => quote! {},
            Type::Null(_typ) => quote! { #ident: (), },
            Type::Integer(_typ) | Type::Real(_typ) | Type::Text(_typ) => quote! { #ident: #ident, },
            Type::Json(_typ) => {
                let serde_err = format!("Dao Value of {ident} was not valid Json");
                // Bit of an issue treating Sql Json columns as json objects, rather than strings
                quote! {
                    #ident: serde_json::from_str(#ident.as_str() // If this was a string i.e. Json
                            .map(|s| s.to_string()) // Type manipulation
                            .unwrap_or_else(|| #ident.to_string()) // If this was a number etc.
                            .as_str()) // Type manipulation
                        .expect(#serde_err),
                }
            }
        }
    }
}
impl From<Field> for Column {
    fn from(field: Field) -> Self {
        Self {
            ident: field.ident.clone().unwrap_or_else(|| {
                Diagnostic::spanned(
                    vec![Span::call_site(), field.span().unwrap()],
                    Level::Error,
                    format!("No Ident on Field {:?} in struct", field.ident),
                )
                .emit();
                panic!("No Ident on Field in struct")
            }),
            typ: field.ty.into(),
        }
    }
}

use std::{collections::BTreeMap, vec::Vec};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Fields, Ident, ItemStruct, Type as SynType};

#[derive(PartialEq, Eq)]
pub enum Type {
    PrimaryKey,
    Null(()),
    Integer(Ident),
    Real(Ident),
    Text(Ident),
    Json(Ident),
}
impl From<SynType> for Type {
    fn from(value: SynType) -> Self {
        if let SynType::Path(tp) = value {
            let ident = tp
                .path
                .get_ident()
                .expect("Field Type Path is not Some")
                .clone();
            let segments = &tp.path.segments;
            if segments.len() == 1 {
                let typ = format!("{}", segments[0].ident);
                return match typ.as_str() {
                    "()" => Self::Null(()),
                    "bool" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" => {
                        Self::Integer(ident)
                    }
                    "f32" | "f64" => Self::Real(ident),
                    "String" | "&str" | "&[u8]" => Self::Text(ident),
                    _ => Self::Json(ident),
                };
            }
            return Self::Json(ident);
        }
        panic!("Field Type is not a TypePath")
    }
}
impl Type {
    fn as_table_defn(&self) -> &'static str {
        match self {
            Self::PrimaryKey => "integer primary key autoincrement",
            Self::Null(_typ) => "null",
            Self::Integer(_typ) => "integer",
            Self::Real(_typ) => "real",
            Self::Text(_typ) => "text",
            Self::Json(_typ) => "json",
        }
    }

    fn as_tosql(&self) -> TokenStream {
        match self {
            Self::PrimaryKey => quote! { Option<i64> },
            Self::Null(_typ) => quote! { () },
            Self::Integer(typ) | Self::Real(typ) | Self::Text(typ) => quote! { #typ },
            Self::Json(_typ) => quote! { serde_json::Value },
        }
    }

    fn as_method_arg(&self) -> TokenStream {
        match self {
            Self::PrimaryKey => quote! { i64 },
            Self::Null(_typ) => quote! { () },
            Self::Integer(typ) | Self::Real(typ) | Self::Text(typ) | Self::Json(typ) => {
                quote! { #typ }
            }
        }
    }
}

struct Column {
    ident: Ident,
    typ: Type,
}
impl Column {
    fn ident_key(&self) -> String {
        format!(":{}", self.ident)
    }

    fn as_table_defn(&self) -> String {
        format!("{} {}", self.ident, self.typ.as_table_defn())
    }

    fn as_serde_value(&self) -> TokenStream {
        let ident = &self.ident;
        let ident_key = self.ident_key();
        match self.typ {
            Type::Json(_) => quote! { #ident_key: serde_json::to_value(#ident).unwrap() },
            _ => quote! { #ident_key: #ident },
        }
    }

    fn as_method_arg(&self) -> TokenStream {
        let ident = &self.ident;
        let typ = self.typ.as_method_arg();
        quote! { #ident: &#typ }
    }

    fn select(&self, table_name: &String) -> TokenStream {
        let method_arg = self.as_method_arg();
        let select_by_ident = format_ident!("select_by_{}", self.ident);
        let select_sql = format!(
            "select * from {} where {} = {}",
            table_name, self.ident, self.ident_key()
        );
        let serde_expr = self.as_serde_value();

        quote! {
            pub fn #select_by_ident(conn: &rusqlite::Connection, #method_arg) -> rusqlite::Result<std::vec::Vec<Self>> {
                Ok(conn.prepare(#select_sql)?
                    .query_and_then(rusqlite::named_params! {#serde_expr}, serde_rusqlite::from_row::<Self>)?
                    .map(|row| row.unwrap())
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
                    .query_and_then(rusqlite::named_params! {#serde_expr}, serde_rusqlite::from_row::<i64>)?
                    .map(|row| row.unwrap())
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

    fn as_orm_methods(&self, ident_id: &Ident, table_name: &String) -> TokenStream {
        let select = self.select(table_name);
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

    fn as_struct_defn(&self) -> TokenStream {
        let Self { ident, typ } = self;
        let typ = typ.as_tosql();

        quote! {
            #ident: #typ,
        }
    }

    fn as_into_value(&self) -> TokenStream {
        let Self { ident, typ } = self;
        match typ {
            Type::PrimaryKey => quote! { #ident: None, },
            Type::Null(_typ) => quote! { (), },
            Type::Integer(_typ) | Type::Real(_typ) | Type::Text(_typ) => quote! { #ident: #ident, },
            Type::Json(_typ) => quote! {
                #ident: serde_json::to_value(#ident).unwrap(),
            },
        }
    }

    fn as_from_value(&self) -> TokenStream {
        let Self { ident, typ } = self;
        match typ {
            Type::PrimaryKey => quote! {},
            Type::Null(_typ) => quote! { #ident: (), },
            Type::Integer(_typ) | Type::Real(_typ) | Type::Text(_typ) => quote! { #ident: #ident, },
            Type::Json(_typ) => quote! {
                #ident: serde_json::from_value(#ident).unwrap(),
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

pub struct Dao {
    ident: Ident,
    columns: Vec<Column>,
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

    fn as_serde_values(&self) -> TokenStream {
        let arguments = self
            .columns
            .iter()
            .map(Column::as_serde_value)
            .collect::<Vec<_>>();
        quote! { #(#arguments),* }
    }

    fn create(&self, table_name: &String) -> TokenStream {
        let table_defn = self.as_table_defn();
        let create_sql = format!("create table {}({})", table_name, table_defn);

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
        let serde_exprs = self.as_serde_values();
        let dao_fieldnames = TokenStream::from_iter(
            self.columns
                .iter()
                .map(|Column { ident, .. }| quote! { #ident, }),
        );

        quote! {
            pub fn insert(self, conn: &rusqlite::Connection) -> rusqlite::Result<#ident_id> {
                let Self {
                    #dao_fieldnames
                } = self;
                conn.prepare(#update_sql)?
                    .execute(rusqlite::named_params! {#serde_exprs})?;
                Ok(#ident_id(conn.last_insert_rowid()))
            }
        }
    }

    pub fn as_tokenstream(&self) -> TokenStream {
        let Self { ident, columns } = self;
        let table_name = format!("{}s", ident).to_lowercase();
        let ident_id = format_ident!("{}Id", ident);
        let ident_dao = format_ident!("{}Dao", ident);

        let create = self.create(&table_name);
        let insert = self.insert(&ident_id, &table_name);
        let fields = TokenStream::from_iter(columns.iter().map(Column::as_struct_defn));
        let methods = TokenStream::from_iter(
            columns
                .iter()
                .map(|col| col.as_orm_methods(&ident_id, &table_name)),
        );
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
        }
    }

    pub fn as_map(&self) -> BTreeMap<&Ident, &Type> {
        self.columns
            .iter()
            .map(|col| (&col.ident, &col.typ))
            .collect::<BTreeMap<_, _>>()
    }
}

impl From<ItemStruct> for Dao {
    fn from(value: ItemStruct) -> Self {
        let ident = value.ident;
        let mut columns = match value.fields {
            Fields::Named(fs) => fs,
            _ => panic!("Not a NamedFields struct"),
        }
        .named
        .into_iter()
        .map(|field| field.into())
        .collect::<Vec<Column>>();

        let mut cols_with_id = vec![Column {
            ident: format_ident!("id"),
            typ: Type::PrimaryKey,
        }];
        cols_with_id.append(&mut columns);

        Self {
            ident,
            columns: cols_with_id,
        }
    }
}

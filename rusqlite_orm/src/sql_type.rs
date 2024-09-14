use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type as SynType};

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
    pub fn as_table_defn(&self) -> &'static str {
        match self {
            Self::PrimaryKey => "integer primary key autoincrement",
            Self::Null(_typ) => "null",
            Self::Integer(_typ) => "integer",
            Self::Real(_typ) => "real",
            Self::Text(_typ) => "text",
            Self::Json(_typ) => "json",
        }
    }

    pub fn as_tosql(&self) -> TokenStream {
        match self {
            Self::PrimaryKey => quote! { Option<i64> },
            Self::Null(_typ) => quote! { () },
            Self::Integer(typ) | Self::Real(typ) | Self::Text(typ) => quote! { #typ },
            Self::Json(_typ) => quote! { serde_json::Value },
        }
    }

    pub fn as_method_arg(&self) -> TokenStream {
        match self {
            Self::PrimaryKey => quote! { i64 },
            Self::Null(_typ) => quote! { () },
            Self::Integer(typ) | Self::Real(typ) | Self::Text(typ) | Self::Json(typ) => {
                quote! { #typ }
            }
        }
    }
}

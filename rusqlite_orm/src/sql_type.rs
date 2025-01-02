use proc_macro2::TokenStream;
use quote::quote;
use syn::Type as SynType;

pub enum Type {
    PrimaryKey,
    Null(()),
    Integer(TokenStream),
    Real(TokenStream),
    Text(TokenStream),
    Json(TokenStream),
}
impl Type {
    fn from_syn(root: TokenStream, value: SynType) -> Self {
        match value {
            SynType::Path(path) => {
                let segments = &path.path.segments;
                if let Some(_ident) = path.path.get_ident() {
                    let typ = format!("{}", segments[0].ident);
                    match typ.as_str() {
                        "()" => Self::Null(()),
                        "bool" => Self::Integer(root),
                        "i8" | "i16" | "i32" | "i64" | "i128" => Self::Integer(root),
                        "u8" | "u16" | "u32" | "u64" | "u128" => Self::Integer(root),
                        "isize" | "usize" => Self::Integer(root),
                        "f32" | "f64" => Self::Real(root),
                        "String" | "str" | "char" => Self::Text(root),
                        _ => Self::Json(root),
                    }
                } else {
                    Self::Json(root)
                }
            }
            SynType::Slice(slice) => match *slice.elem {
                SynType::Path(path)
                    if matches!(
                        format!("{}", path.path.segments[0].ident).as_str(),
                        "char" | "u8" | "i8"
                    ) =>
                {
                    Self::Text(root)
                }
                _ => Self::Json(root),
            },
            SynType::Array(array) => match *array.elem {
                SynType::Path(path)
                    if matches!(
                        format!("{}", path.path.segments[0].ident).as_str(),
                        "char" | "u8" | "i8"
                    ) =>
                {
                    Self::Text(root)
                }
                _ => Self::Json(root),
            },
            SynType::Group(group) => Self::from_syn(root, *group.elem),
            SynType::Paren(paren) => Self::from_syn(root, *paren.elem),
            SynType::Reference(refer) => Self::from_syn(root, *refer.elem),
            _ => Self::Json(root),
        }
    }
}
impl From<SynType> for Type {
    fn from(value: SynType) -> Self {
        Self::from_syn(quote! { #value }, value)
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
            Self::Integer(typ) | Self::Real(typ) | Self::Text(typ) => quote! { #typ },
            Self::Json(seg) => quote! { #seg },
        }
    }
}

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(PrestoVarchar)]
pub fn derive_presto_varchar(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPrestoAttribute for #name {
            fn get_presto_type(&self) -> String {
                        "VARCHAR".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PrestoBigint)]
pub fn derive_presto_bigint(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPrestoAttribute for #name {
            fn get_presto_type(&self) -> String {
                        "BIGINT".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PrestoReal)]
pub fn derive_presto_real(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPrestoAttribute for #name {
            fn get_presto_type(&self) -> String {
                        "REAL".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PrestoRegressor)]
pub fn derive_presto_regressor(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPrestoAttribute for #name {
            fn get_presto_type(&self) -> String {
                        "REGRESSOR".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PrestoDouble)]
pub fn derive_presto_double(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPrestoAttribute for #name {
            fn get_presto_type(&self) -> String {
                        "DOUBLE".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(OrcString)]
pub fn derive_orc_string(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TOrcAttribute for #name {
            fn get_orc_type(&self) -> String {
                        "STRING".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(OrcBigint)]
pub fn derive_orc_bigint(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TOrcAttribute for #name {
            fn get_orc_type(&self) -> String {
                        "BIGINT".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(OrcFloat)]
pub fn derive_orc_float(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TOrcAttribute for #name {
            fn get_orc_type(&self) -> String {
                        "FLOAT".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SQLVarchar)]
pub fn derive_sql_varchar(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TSQLAttribute for #name {
            fn get_sql_type(&self) -> DataType {
                DataType::Varchar(None)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SQLBigint)]
pub fn derive_sql_bigint(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TSQLAttribute for #name {
            fn get_sql_type(&self) -> DataType {
                DataType::BigInt
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SQLReal)]
pub fn derive_sql_real(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TSQLAttribute for #name {
            fn get_sql_type(&self) -> DataType {
                DataType::Real
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SQLiteInteger)]
pub fn derive_sqlite_integer(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TSQLiteAttribute for #name {
            fn get_sqlite_type(&self) -> String {
                "INTEGER".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SQLiteReal)]
pub fn derive_sqlite_real(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TSQLiteAttribute for #name {
            fn get_sqlite_type(&self) -> String {
                "REAL".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SQLiteText)]
pub fn derive_sqlite_text(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TSQLiteAttribute for #name {
            fn get_sqlite_type(&self) -> String {
                "TEXT".to_string()
            }
        }
    };
    gen.into()
}

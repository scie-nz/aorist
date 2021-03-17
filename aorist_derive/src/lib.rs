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

#[proc_macro_derive(PostgresSmallInt)]
pub fn derive_postgres_smallint(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "SMALLINT".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresInteger)]
pub fn derive_postgres_integer(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "INTEGER".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresBigInt)]
pub fn derive_postgres_bigint(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "BIGINT".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresDecimal)]
pub fn derive_postgres_decimal(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "DECIMAL".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresNumeric)]
pub fn derive_postgres_numeric(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "NUMERIC".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresReal)]
pub fn derive_postgres_real(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "REAL".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresDoublePrecision)]
pub fn derive_postgres_doubleprecision(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "DOUBLE PRECISION".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresSmallSerial)]
pub fn derive_postgres_smallserial(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "SMALLSERIAL".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresSerial)]
pub fn derive_postgres_serial(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "SERIAL".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresBigSerial)]
pub fn derive_postgres_bigserial(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "BIGSERIAL".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresMoney)]
pub fn derive_postgres_money(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "MONEY".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresVarchar)]
pub fn derive_postgres_varchar(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "VARCHAR".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresChar)]
pub fn derive_postgres_char(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "CHAR".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresText)]
pub fn derive_postgres_text(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "TEXT".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresBytea)]
pub fn derive_postgres_bytea(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "BYTEA".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresTimestamp)]
pub fn derive_postgres_timestamp(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "TIMESTAMP".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresDate)]
pub fn derive_postgres_date(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "DATE".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresTime)]
pub fn derive_postgres_time(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "TIME".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresInterval)]
pub fn derive_postgres_interval(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "INTERVAL".to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresBoolean)]
pub fn derive_postgres_boolean(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> String {
                "BOOLEAN".to_string()
            }
        }
    };
    gen.into()
}

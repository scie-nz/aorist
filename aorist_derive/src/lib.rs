extern crate proc_macro;
use aorist_util::{extract_type_from_linked_hash_map, extract_type_from_vector};
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Data, DataStruct, DeriveInput, Field, Fields};

#[proc_macro_derive(PrestoVarchar)]
pub fn derive_presto_varchar(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPrestoAttribute for #name {
            fn get_presto_type(&self) -> AString {
                        "VARCHAR".into()
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
            fn get_presto_type(&self) -> AString {
                        "BIGINT".into()
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
            fn get_presto_type(&self) -> AString {
                        "REAL".into()
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
            fn get_presto_type(&self) -> AString {
                        "REGRESSOR".into()
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
            fn get_presto_type(&self) -> AString {
                        "DOUBLE".into()
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
            fn get_orc_type(&self) -> AString {
                        "STRING".into()
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
            fn get_orc_type(&self) -> AString {
                        "BIGINT".into()
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
            fn get_orc_type(&self) -> AString {
                        "FLOAT".into()
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
            fn get_sqlite_type(&self) -> AString {
                "INTEGER".into()
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
            fn get_sqlite_type(&self) -> AString {
                "REAL".into()
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
            fn get_sqlite_type(&self) -> AString {
                "TEXT".into()
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
            fn get_postgres_type(&self) -> AString {
                "SMALLINT".into()
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
            fn get_postgres_type(&self) -> AString {
                "INTEGER".into()
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
            fn get_postgres_type(&self) -> AString {
                "BIGINT".into()
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
            fn get_postgres_type(&self) -> AString {
                "DECIMAL".into()
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
            fn get_postgres_type(&self) -> AString {
                "NUMERIC".into()
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
            fn get_postgres_type(&self) -> AString {
                "REAL".into()
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
            fn get_postgres_type(&self) -> AString {
                "DOUBLE PRECISION".into()
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
            fn get_postgres_type(&self) -> AString {
                "SMALLSERIAL".into()
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
            fn get_postgres_type(&self) -> AString {
                "SERIAL".into()
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
            fn get_postgres_type(&self) -> AString {
                "BIGSERIAL".into()
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
            fn get_postgres_type(&self) -> AString {
                "MONEY".into()
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
            fn get_postgres_type(&self) -> AString {
                "VARCHAR".into()
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
            fn get_postgres_type(&self) -> AString {
                "CHAR".into()
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
            fn get_postgres_type(&self) -> AString {
                "TEXT".into()
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
            fn get_postgres_type(&self) -> AString {
                "BYTEA".into()
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
            fn get_postgres_type(&self) -> AString {
                "TIMESTAMP".into()
            }
            fn psycopg2_value_json_serializable(&self) -> bool {
                false
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
            fn get_postgres_type(&self) -> AString {
                "DATE".into()
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
            fn get_postgres_type(&self) -> AString {
                "TIME".into()
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
            fn get_postgres_type(&self) -> AString {
                "INTERVAL".into()
            }
            // not JSON-serializable (TODO GeoJSON?)
            fn psycopg2_value_json_serializable(&self) -> bool {
                false
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
            fn get_postgres_type(&self) -> AString {
                "BOOLEAN".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryBool)]
pub fn derive_bigquery_bool(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "BOOL".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryBytes)]
pub fn derive_bigquery_bytes(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "BYTES".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryDate)]
pub fn derive_bigquery_date(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "DATE".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryDateTime)]
pub fn derive_bigquery_datetime(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "DATETIME".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryGeography)]
pub fn derive_bigquery_geography(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "GEOGRAPHY".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryInt64)]
pub fn derive_bigquery_int64(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "INT64".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryNumeric)]
pub fn derive_bigquery_numeric(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "NUMERIC".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryBigNumeric)]
pub fn derive_bigquery_bignumeric(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "BIGNUMERIC".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryFloat64)]
pub fn derive_bigquery_float64(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "FLOAT64".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryString)]
pub fn derive_bigquery_string(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "STRING".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryTime)]
pub fn derive_bigquery_time(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "TIME".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BigQueryTimeStamp)]
pub fn derive_bigquery_timestamp(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TBigQueryAttribute for #name {
            fn get_bigquery_type(&self) -> AString {
                "TIMESTAMP".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresCharacterVarying)]
pub fn derive_postgres_character_varying(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "CHARACTER VARYING".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresTimestampWithoutTimeZone)]
pub fn derive_postgres_timestamp_without_time_zone(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "TIMESTAMP WITHOUT TIME ZONE".into()
            }
            fn psycopg2_value_json_serializable(&self) -> bool {
                false
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresUuid)]
pub fn derive_postgres_uuid(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "UUID".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresOid)]
pub fn derive_postgres_oid(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "OID".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresName)]
pub fn derive_postgres_name(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "NAME".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresTimestampWithTimeZone)]
pub fn derive_postgres_timestamp_with_time_zone(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "TIMESTAMP WITH TIME ZONE".into()
            }
            fn psycopg2_value_json_serializable(&self) -> bool {
                false
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresJSONB)]
pub fn derive_postgres_jsonb(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "JSONB".into()
            }
            // not JSON-serializable
            fn psycopg2_value_json_serializable(&self) -> bool {
                false
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresUserDefined)]
pub fn derive_postgres_userdefined(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "USER-DEFINED".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresArray)]
pub fn derive_postgres_array(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let array = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #array {
            fn get_postgres_type(&self) -> AString {
                "ARRAY".into()
            }
            // not JSON-serializable
            fn psycopg2_value_json_serializable(&self) -> bool {
                false
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PostgresRegProc)]
pub fn derive_postgres_regproc(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "REGPROC".into()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(PostgresPgNodeTree)]
pub fn derive_postgres_pgnodetree(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "PG_NODE_TREE".into()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(PostgresPgLsn)]
pub fn derive_postgres_pglsn(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "PG_LSN".into()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(PostgresXid)]
pub fn derive_postgres_xid(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "XID".into()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(PostgresAnyArray)]
pub fn derive_postgres_anyarray(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "ANYARRAY".into()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(PostgresRegType)]
pub fn derive_postgres_regtype(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "REGTYPE".into()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(PostgresPgNDistinct)]
pub fn derive_postgres_pgndistinct(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "PG_NDISTINCT".into()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(PostgresPgDependencies)]
pub fn derive_postgres_pgdependencies(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "PG_DEPENDENCIES".into()
            }
        }
    };
    gen.into()
}
#[proc_macro_derive(PostgresInet)]
pub fn derive_postgres_inet(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "INET".into()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Optimizable)]
pub fn optimizable(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    //let constraint_names = AoristConstraint::get_required_constraint_names();
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => optimize_struct_fields(&fields.named, &input),
        _ => panic!("expected a struct with named fields"),
    }
}

fn optimize_struct_fields(fields: &Punctuated<Field, Comma>, input: &DeriveInput) -> TokenStream {
    let bare_field_name = fields
        .iter()
        .filter(|field| match &field.ty {
            syn::Type::Path(x) => x.path.is_ident("AST"),
            _ => false,
        })
        .map(|field| &field.ident);

    let vec_field_name = fields
        .iter()
        .map(|field| (&field.ident, extract_type_from_vector(&field.ty)))
        .filter(|x| match x.1 {
            Some(syn::Type::Path(y)) => y.path.is_ident("AST"),
            _ => false,
        })
        .map(|x| x.0);

    let map_field_name = fields
        .iter()
        .map(|field| (&field.ident, extract_type_from_linked_hash_map(&field.ty)))
        .filter(|x| match x.1 {
            Some((_, syn::Type::Path(y))) => y.path.is_ident("AST"),
            _ => false,
        })
        .map(|x| x.0);

    let struct_name = &input.ident;
    TokenStream::from(quote! {

    impl #struct_name {
        fn optimize_fields(&mut self) {
            #(
                if let Some(opt) = self.#bare_field_name.optimize() {
                    self.#bare_field_name = opt;
                }
                self.#bare_field_name.optimize_fields();
            )*
            #(
                let mut new_elems = AVec::new();
                for elem in self.#vec_field_name {
                    let new_elem = match elem.optimize() {
                        Some(opt) => opt,
                        None => elem.clone(),
                    };
                    new_elem.optimize_fields();
                    new_elems.push(new_elem);
                }
                self.#vec_field_name = new_elems;
            )*

            #(
                let mut new_elems = LinkedHashMap::new();
                for (k, elem) in self.#map_field_name {
                    let new_elem = match elem.optimize() {
                        Some(opt) => opt,
                        None => elem.clone(),
                    };
                    new_elem.optimize_fields();
                    new_elems.insert(k, new_elem);
                }
                self.#map_field_name = new_elems;
            )*
        }
    }
    })
}

#[proc_macro_derive(PostgresGeometry)]
pub fn derive_postgres_geometry(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TPostgresAttribute for #name {
            fn get_postgres_type(&self) -> AString {
                "GEOMETRY".into()
            }
            // not JSON-serializable
            fn psycopg2_value_json_serializable(&self) -> bool {
                false
            }
        }
    };
    gen.into()
}

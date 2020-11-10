extern crate proc_macro;
use quote::quote;
use syn;
use proc_macro::TokenStream;

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

#[proc_macro_derive(BlankPrefectPreamble)]
pub fn derive_blank_prefect_preamble(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
	  let gen = quote! {
        impl TObjectWithPrefectCodeGen for #name {
            fn get_prefect_preamble(&self, _preamble: &mut HashMap<String, String>) {
            }
        }
    };
	gen.into()
}

#[proc_macro_derive(NoPythonImports)]
pub fn derive_no_python_imports(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
	  let gen = quote! {
        impl TObjectWithPythonCodeGen for #name {
            fn get_python_imports(&self, _preamble: &mut HashMap<String, String>) {
            }
        }
    };
	gen.into()
}

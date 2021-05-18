extern crate proc_macro;
use self::proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::FieldsNamed;
mod keyword {
    syn::custom_keyword!(path);
}
pub trait Builder {
    fn new(fields: &FieldsNamed) -> Self; 
    fn to_file(&self, struct_name: &Ident, file_name: &str);
    fn to_concept_token_stream(&self, struct_name: &Ident) -> TokenStream;
    fn to_python_token_stream(&self, struct_name: &Ident) -> TokenStream;
    fn to_base_token_stream(&self, struct_name: &Ident) -> TokenStream;
}

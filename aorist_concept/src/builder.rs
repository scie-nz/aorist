extern crate proc_macro;
use self::proc_macro::TokenStream;
use proc_macro2::Ident;
mod keyword {
    syn::custom_keyword!(path);
}
pub trait Builder {
    type TInput;
    fn new(fields: &Self::TInput) -> Self; 
    fn to_file(&self, struct_name: &Ident, file_name: &str);
    fn to_concept_token_stream(&self, struct_name: &Ident) -> TokenStream;
    fn to_python_token_stream(&self, struct_name: &Ident) -> TokenStream;
    fn to_base_token_stream(&self, struct_name: &Ident) -> TokenStream;
}

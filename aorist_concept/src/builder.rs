extern crate proc_macro;
use self::proc_macro::TokenStream;
use aorist_util::AoristError;
use proc_macro2::Ident;
mod keyword {
    syn::custom_keyword!(path);
}
pub trait Builder {
    type TInput;
    fn new(fields: &Self::TInput) -> Result<Self, AoristError>
    where
        Self: Sized;
    fn to_file(&self, struct_name: &Ident, file_name: &str) -> Result<(), AoristError>;
    fn to_concept_token_stream(&self, struct_name: &Ident) -> Result<TokenStream, AoristError>;
    fn to_concept_children_token_stream(
        &self,
        struct_name: &Ident,
    ) -> Result<TokenStream, AoristError>;
}

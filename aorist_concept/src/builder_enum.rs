extern crate proc_macro;
use aorist_util::AoristError;
use self::proc_macro::TokenStream;
use crate::builder::Builder;
use crate::struct_builder::StructBuilder;
use crate::enum_builder::EnumBuilder;
use proc_macro2::Ident;
mod keyword {
    syn::custom_keyword!(path);
}
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields};

pub enum BuilderEnum {
    StructBuilder(StructBuilder, Ident),
    EnumBuilder(EnumBuilder, Ident),
}
impl BuilderEnum {
    pub fn get_name(&self) -> &Ident {
        match &self {
            Self::StructBuilder(ref _b, ref name) => name,
            Self::EnumBuilder(ref _b, ref name) => name,
        }
    }
    #[allow(dead_code)]
    pub fn to_file(&self, file_name: &str) -> Result<(), AoristError> {
        match &self {
            Self::StructBuilder(ref b, ref name) => b.to_file(name, file_name),
            Self::EnumBuilder(ref b, ref name) => b.to_file(name, file_name),
        }
    }
    pub fn to_concept_token_stream(&self) -> Result<TokenStream, AoristError> {
        match &self {
            Self::StructBuilder(ref b, ref name) => b.to_concept_token_stream(name),
            Self::EnumBuilder(ref b, ref name) => b.to_concept_token_stream(name),
        }
    }
    pub fn to_concept_children_token_stream(&self) -> Result<TokenStream, AoristError> {
        match &self {
            Self::StructBuilder(ref b, ref name) => b.to_concept_children_token_stream(name),
            Self::EnumBuilder(ref b, ref name) => b.to_concept_children_token_stream(name),
        }
    }
    
    pub fn new(input: DeriveInput) -> Self {
        let (_name, builder_res) = match &input.data {
            Data::Struct(DataStruct {
                fields: Fields::Named(ref fields),
                ..
            }) => {
                let struct_name = &input.ident;
                let builder = StructBuilder::new(fields);
                (struct_name, match builder {
                    Ok(b) => Ok(BuilderEnum::StructBuilder(b, struct_name.clone())),
                    Err(err) => Err(err),
                })
            }
            Data::Enum(DataEnum { variants, .. }) => {
                let enum_name = &input.ident;
                let builder = EnumBuilder::new(variants);
                (enum_name, match builder {
                    Ok(b) => Ok(BuilderEnum::EnumBuilder(b, enum_name.clone())),
                    Err(err) => Err(err),
                })
            }
            _ => panic!("expected a struct with named fields or an enum"),
        };
        match builder_res {
            Ok(x) => x,
            Err(err) => panic!("Cannot create builder for {}: {:?}", &input.ident, err),
        }
    }
}

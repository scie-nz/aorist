// Following: https://github.com/dtolnay/syn/issues/516
extern crate proc_macro;
mod builder;
mod concept_builder;
mod enum_builder;
mod struct_builder;

use self::proc_macro::TokenStream;
use crate::builder::Builder;
use crate::concept_builder::{ConceptBuilder, RawConceptBuilder, TConceptBuilder};
use crate::enum_builder::EnumBuilder;
use crate::struct_builder::StructBuilder;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};
mod keyword {
    syn::custom_keyword!(path);
}
use std::sync::Arc;

#[proc_macro_derive(ConstrainableWithChildren)]
pub fn constrainable_with_children(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            let struct_name = &input.ident;
            let builder = StructBuilder::new(fields);
            //builder.to_file(struct_name, "constrainables.txt");
            builder.to_concept_children_token_stream(struct_name)
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let enum_name = &input.ident;
            let builder = EnumBuilder::new(variants);
            //builder.to_file(enum_name, "constraints.txt");
            builder.to_concept_children_token_stream(enum_name)
        }
        _ => panic!("expected a struct with named fields or an enum"),
    }
}

#[proc_macro_derive(InnerObject, attributes(py_default))]
pub fn constrain_object(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match &ast.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let enum_name = &ast.ident;
            let builder = EnumBuilder::new(variants);
            let _base_stream = builder.to_base_token_stream(enum_name);
            let python_stream = builder.to_python_token_stream(enum_name);
            //base_stream.into_iter().chain(python_stream.into_iter()).collect()
            python_stream
        }
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let struct_name = &ast.ident;
            let builder = StructBuilder::new(&fields);
            let _base_stream = builder.to_base_token_stream(struct_name);
            let python_stream = builder.to_python_token_stream(struct_name);
            //base_stream.into_iter().chain(python_stream.into_iter()).collect()
            python_stream
        }

        _ => panic!("expected a struct with named fields or an enum"),
    }
}
#[proc_macro_attribute]
pub fn aorist_concept(args: TokenStream, input: TokenStream) -> TokenStream {
    let builder = ConceptBuilder::new(vec![
        "InnerObject",
        "Constrainable",
        "ConstrainableWithChildren",
    ]);
    builder.gen(args, input)
}
#[proc_macro_derive(InnerObjectNew, attributes(py_default))]
pub fn inner_object_new(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match &ast.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let enum_name = &ast.ident;
            let builder = EnumBuilder::new(variants);
            let _base_stream = builder.to_base_token_stream(enum_name);
            let python_stream = builder.to_python_token_stream_new(enum_name);
            python_stream
        }
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let struct_name = &ast.ident;
            let builder = StructBuilder::new(&fields);
            let _base_stream = builder.to_base_token_stream(struct_name);
            let python_stream = builder.to_python_token_stream_new(struct_name);
            python_stream
        }

        _ => panic!("expected a struct with named fields or an enum"),
    }
}
#[proc_macro_attribute]
pub fn aorist(args: TokenStream, input: TokenStream) -> TokenStream {
    let builder = RawConceptBuilder::new(vec![
        "aorist_concept::Constrainable",
        //"aorist_concept::ConstrainableWithChildren",
    ]);
    builder.gen_new(args, input)
}

#[proc_macro_derive(Constrainable, attributes(constrainable))]
pub fn constrainable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            let struct_name = &input.ident;
            let builder = StructBuilder::new(fields);
            //builder.to_file(struct_name, "constrainables.txt");
            builder.to_concept_token_stream2(struct_name)
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let enum_name = &input.ident;
            let builder = EnumBuilder::new(variants);
            //builder.to_file(enum_name, "constraints.txt");
            builder.to_concept_token_stream2(enum_name)
        }
        _ => panic!("expected a struct with named fields or an enum"),
    }
}

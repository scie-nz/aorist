// Following: https://github.com/dtolnay/syn/issues/516
extern crate proc_macro;
use self::proc_macro::TokenStream;
use std::fs::OpenOptions;
use std::io::prelude::*;
use type_macro_helpers::{extract_type_from_option, extract_type_from_vector};

use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Meta, Type, Variant,
};
mod keyword {
    syn::custom_keyword!(path);
}

fn process_enum_variants(
    variants: &Punctuated<Variant, Comma>,
    input: &DeriveInput,
) -> TokenStream {
    let enum_name = &input.ident;
    let variant = variants.iter().map(|x| (&x.ident));
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("constrainables.txt")
        .unwrap();
    for v in variant.clone() {
        writeln!(file, "'{}'->'{}';", enum_name, v).unwrap();
    }
    TokenStream::from(quote! {
      impl AoristConcept for #enum_name {
        fn traverse_constrainable_children(&self) {
            match self {
              #(
                #enum_name::#variant(x) => x.traverse_constrainable_children(),
              )*
            }
        }
      }
    })
}
fn process_struct_fields(fields: &Punctuated<Field, Comma>, input: &DeriveInput) -> TokenStream {
    let field = fields
        .iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .filter(|a| match a.parse_meta() {
                    Ok(Meta::Path(x)) => x.is_ident("constrainable"),
                    _ => false,
                })
                .collect::<Vec<_>>()
                .len()
                > 0
        })
        .map(|field| (&field.ident, &field.ty));

    let struct_name = &input.ident;
    let bare_field = field.clone().filter(|x| {
        extract_type_from_option(x.1).is_none() && extract_type_from_vector(x.1).is_none()
    });
    let option_field = field
        .clone()
        .filter(|x| extract_type_from_option(x.1).is_some())
        .map(|x| (x.0, extract_type_from_option(x.1).unwrap()));
    let vec_field = field.clone().filter(|x| {
        extract_type_from_option(x.1).is_none() && extract_type_from_vector(x.1).is_some()
    }).map(|x| (x.0, extract_type_from_vector(x.1).unwrap()));
    let option_vec_field = option_field
        .clone()
        .filter(|x| extract_type_from_vector(x.1).is_some())
        .map(|x| (x.0, extract_type_from_vector(x.1).unwrap()));
    let types = bare_field
        .clone()
        .map(|x| x.1)
        .chain(option_vec_field.clone().map(|x| x.1))
        .chain(vec_field.clone().map(|x| x.1));
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("constrainables.txt")
        .unwrap();
    /*writeln!(
        file,
        "{}: {} total, {} bare types, {} vec types, {} option_vec_types",
        struct_name,
        field.clone().collect::<Vec<_>>().len(),
        bare_field.clone().collect::<Vec<_>>().len(),
        vec_field.clone().collect::<Vec<_>>().len(),
        option_vec_field.clone().collect::<Vec<_>>().len()
    ).unwrap();*/
    for t in types {
        let tp = match t {
            Type::Path(x) => &x.path,
            _ => panic!("Something other than a type path found."),
        };
        let type_val = tp
            .segments
            .iter()
            .map(|x| x.ident.to_string())
            .collect::<Vec<_>>()
            .join("|");
        writeln!(file, "'{}'->'{}';", struct_name, type_val).unwrap();
    }
    let bare_field_name = bare_field.map(|x| x.0);
    let vec_field_name = vec_field.map(|x| x.0);
    let option_vec_field_name = option_vec_field.map(|x| x.0);

    TokenStream::from(quote! {

        impl AoristConcept for #struct_name {
            fn traverse_constrainable_children(&self) {
                #(
                    self.#bare_field_name.traverse_constrainable_children();
                )*
                #(
                    for x in &self.#vec_field_name {
                        x.traverse_constrainable_children();
                    }
                )*
                #(
                    if let Some(ref v) = self.#option_vec_field_name {
                        for x in v {
                            x.traverse_constrainable_children()
                        }
                    }
                )*
            }
        }
    })
}

#[proc_macro_derive(Constrainable, attributes(constrainable))]
pub fn aorist_concept(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => process_struct_fields(&fields.named, &input),
        Data::Enum(DataEnum { variants, .. }) => process_enum_variants(variants, &input),
        _ => panic!("expected a struct with named fields"),
    }
}

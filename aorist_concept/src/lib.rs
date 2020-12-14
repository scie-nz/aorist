// Following: https://github.com/dtolnay/syn/issues/516
extern crate proc_macro;
use self::proc_macro::TokenStream;
use type_macro_helpers::{extract_type_from_option, extract_type_from_vector};

use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Meta,
Field};
use syn::token::Comma;
use syn::punctuated::Punctuated;
mod keyword {
    syn::custom_keyword!(path);
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
    let bare_field = field
        .clone()
        .filter(|x| {
            extract_type_from_option(x.1).is_none() && extract_type_from_vector(x.1).is_none()
        })
        .map(|x| x.0);
    let option_field = field.clone()
        .filter(|x| extract_type_from_option(x.1).is_some())
        .map(|x| (x.0, extract_type_from_option(x.1).unwrap()));
    let vec_field = field.clone()
        .filter(|x| extract_type_from_option(x.1).is_none() &&
        extract_type_from_vector(x.1).is_some())
        .map(|x| x.0);
    let option_vec_field = option_field
        .filter(|x| extract_type_from_vector(x.1).is_some())
        .map(|x| x.0);

    TokenStream::from(quote! {

        impl AoristConcept for #struct_name {
            fn traverse_constrainable_children(&self) {
                #(
                    self.#bare_field.traverse_constrainable_children();
                )*
                #(
                    for x in self.#vec_field {
                        x.traverse_constrainable_children();
                    }
                )*
                #(
                    if let Some(ref v) = self.#option_vec_field {
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
        _ => panic!("expected a struct with named fields"),
    }
}

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
use proc_macro2::{Ident, Span};
mod keyword {
    syn::custom_keyword!(path);
}
use aorist_util::{get_raw_objects_of_type, read_file};
use std::collections::HashMap;

fn process_enum_variants(
    variants: &Punctuated<Variant, Comma>,
    input: &DeriveInput,
) -> TokenStream {
    let enum_name = &input.ident;
    let variant = variants.iter().map(|x| (&x.ident));
    let variant2 = variants.iter().map(|x| (&x.ident));
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("constrainables.txt")
        .unwrap();
    writeln!(
        file,
        "node [shape = box, fillcolor=gray, style=filled, fontname = Helvetica] '{}';",
        enum_name
    )
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

        fn get_constraints(self) -> Vec<Rc<Constraint>> {
          match self {
            #(
              #enum_name::#variant2(x) => x.get_constraints(),
            )*
          }
        }
      }
    })
}
fn process_struct_fields(
    fields: &Punctuated<Field, Comma>,
    input: &DeriveInput,
    constraints: &HashMap<String, Vec<String>>,
) -> TokenStream {
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
    let constraint: Vec<Ident> = match constraints.get(&struct_name.to_string()) {
        Some(v) => v.into_iter().map(|x| Ident::new(x, Span::call_site())).collect(),
        None => Vec::new(),
    };
    let bare_field = field.clone().filter(|x| {
        extract_type_from_option(x.1).is_none() && extract_type_from_vector(x.1).is_none()
    });
    let option_field = field
        .clone()
        .filter(|x| extract_type_from_option(x.1).is_some())
        .map(|x| (x.0, extract_type_from_option(x.1).unwrap()));
    let vec_field = field
        .clone()
        .filter(|x| {
            extract_type_from_option(x.1).is_none() && extract_type_from_vector(x.1).is_some()
        })
        .map(|x| (x.0, extract_type_from_vector(x.1).unwrap()));
    let option_vec_field = option_field
        .clone()
        .filter(|x| extract_type_from_vector(x.1).is_some())
        .map(|x| (x.0, extract_type_from_vector(x.1).unwrap()));
    let types = bare_field
        .clone()
        .chain(option_vec_field.clone())
        .chain(vec_field.clone());
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
    writeln!(
        file,
        "node [shape = oval, fillcolor=white, style=filled, fontname = Helvetica] '{}';",
        struct_name
    )
    .unwrap();
    for (ident, t) in types {
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
        writeln!(
            file,
            "'{}'->'{}' [label='{}'];",
            struct_name,
            type_val,
            ident.as_ref().unwrap()
        )
        .unwrap();
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
            fn get_constraints(self) -> Vec<Rc<Constraint>> {
                let rc = Rc::new(self);
                vec![
                    #(
                        Rc::new(Constraint{
                            name: stringify!(#constraint).to_string(),
                            root: stringify!(#struct_name).to_string(),
                            requires: None,
                            inner: Some(
                                AoristConstraint::#constraint(
                                    crate::constraint::#constraint::new(rc)
                                )
                            ),
                        }),
                    )*
                ]
            }
        }
    })
}

#[proc_macro_derive(Constrainable, attributes(constrainable))]
pub fn aorist_concept(input: TokenStream) -> TokenStream {
    // TODO: this should be passed somehow (maybe env var?)
    let raw_objects = read_file("basic.yaml");
    let constraints = get_raw_objects_of_type(&raw_objects, "Constraint".into());
    // TODO: add dependencies
    let constraints_parsed: Vec<(String, String)> = constraints
        .into_iter()
        .map(|x| (
            x.get("name").unwrap().as_str().unwrap().into(),
            x.get("root").unwrap().as_str().unwrap().into(),
        ))
        .collect();
    let mut constraints_map: HashMap<String, Vec<String>> = HashMap::new();
    for (name, root) in constraints_parsed {
        constraints_map.entry(root).or_insert(Vec::new()).push(name);
    }
    let input = parse_macro_input!(input as DeriveInput);
    //let constraint_names = AoristConstraint::get_required_constraint_names();
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => process_struct_fields(&fields.named, &input, &constraints_map),
        Data::Enum(DataEnum { variants, .. }) => process_enum_variants(variants, &input),
        _ => panic!("expected a struct with named fields"),
    }
}

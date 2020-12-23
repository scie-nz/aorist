// Following: https://github.com/dtolnay/syn/issues/516
extern crate proc_macro;
use self::proc_macro::TokenStream;
use std::fs::OpenOptions;
use std::io::prelude::*;
use type_macro_helpers::{extract_type_from_option, extract_type_from_vector};

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Meta, Type, Variant,
};
mod keyword {
    syn::custom_keyword!(path);
}
use aorist_util::{get_raw_objects_of_type, read_file};
use std::collections::HashMap;

fn process_enum_variants(
    variants: &Punctuated<Variant, Comma>,
    input: &DeriveInput,
    constraints: &HashMap<String, Vec<String>>,
) -> TokenStream {
    let enum_name = &input.ident;
    let constraint: Vec<Ident> = match constraints.get(&enum_name.to_string()) {
        Some(v) => v
            .into_iter()
            .map(|x| Ident::new(x, Span::call_site()))
            .collect(),
        None => Vec::new(),
    };
    let variant = variants.iter().map(|x| (&x.ident));
    let variant2 = variants.iter().map(|x| (&x.ident));
    let variant3 = variants.iter().map(|x| (&x.ident));
    let variant4 = variants.iter().map(|x| (&x.ident));
    let variant5 = variants.iter().map(|x| (&x.ident));
    let variant6 = variants.iter().map(|x| (&x.ident));
    let variant7 = variants.iter().map(|x| (&x.ident));
    let variant8 = variants.iter().map(|x| (&x.ident));
    let variant9 = variants.iter().map(|x| (&x.ident));
    let variant10 = variants.iter().map(|x| (&x.ident));
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
        fn get_child_concepts<'a, 'b>(&'a self) -> Vec<Concept<'b>> where 'a : 'b {
          vec![
              match self {
                #(
                  #enum_name::#variant9(x) => Concept::#variant9(&x),
                )*
              }
          ]
        }
        fn get_tag(&self) -> Option<String> {
            match self {
                #(
                  #enum_name::#variant10(x) => x.get_tag(),
                )*
            }
        }
        fn traverse_constrainable_children(
            &self,
            upstream_constraints: Vec<Arc<RwLock<Constraint>>>
        ) {
          match self {
            #(
              #enum_name::#variant(x) =>
              x.traverse_constrainable_children(upstream_constraints),
            )*
          }
        }

        fn compute_constraints(&mut self) {
          let uuid = self.get_uuid();
          match self {
            #(
              #enum_name::#variant4(ref mut x) => {
                  x.compute_constraints();
              }
            )*
          }
          let downstream = self.get_downstream_constraints();
          let enum_constraints = vec![
            #(
              Arc::new(RwLock::new(Constraint{
                  name: stringify!(#constraint).to_string(),
                  root: stringify!(#enum_name).to_string(),
                  requires: None,
                  inner: Some(
                      AoristConstraint::#constraint(
                          crate::constraint::#constraint::new(
                              uuid.clone(),
                              downstream.clone(),
                          )
                      )
                  ),
              })),
            )*
          ];
          match self {
            #(
              #enum_name::#variant8(ref mut x) => {
                  for el in enum_constraints.into_iter() {
                      x.constraints.push(el);
                  };
              }
            )*
          }
        }

        fn get_constraints(&self) -> &Vec<Arc<RwLock<Constraint>>> {
          match self {
            #(
              #enum_name::#variant2(x) => x.get_constraints(),
            )*
          }
        }

        fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
          match self {
            #(
              #enum_name::#variant5(x) => x.get_downstream_constraints(),
            )*
          }
        }

        fn get_uuid(&self) -> Uuid {
          match self {
            #(
              #enum_name::#variant3(x) => x.get_uuid(),
            )*
          }
        }
        fn get_children_uuid(&self) -> Vec<Uuid> {
          match self {
            #(
              #enum_name::#variant6(x) => x.get_children_uuid(),
            )*
          }
        }
        fn compute_uuids(&mut self) {
          match self {
            #(
              #enum_name::#variant7(x) => x.compute_uuids(),
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
        Some(v) => v
            .into_iter()
            .map(|x| Ident::new(x, Span::call_site()))
            .collect(),
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
    let bare_field_name = bare_field.clone().map(|x| x.0);
    let bare_field_type = bare_field.clone().map(|x| x.1);
    let bare_field_name2 = bare_field_name.clone();
    let bare_field_name3 = bare_field_name.clone();
    let bare_field_name4 = bare_field_name.clone();
    let bare_field_name5 = bare_field_name.clone();
    let bare_field_name6 = bare_field_name.clone();
    let bare_field_name7 = bare_field_name.clone();
    let vec_field_name = vec_field.clone().map(|x| x.0);
    let vec_field_type = vec_field.clone().map(|x| x.1);
    let vec_field_name2 = vec_field_name.clone();
    let vec_field_name3 = vec_field_name.clone();
    let vec_field_name4 = vec_field_name.clone();
    let vec_field_name5 = vec_field_name.clone();
    let vec_field_name6 = vec_field_name.clone();
    let option_vec_field_name = option_vec_field.clone().map(|x| x.0);
    let option_vec_field_type = option_vec_field.clone().map(|x| x.1);
    let option_vec_field_name2 = option_vec_field_name.clone();
    let option_vec_field_name3 = option_vec_field_name.clone();
    let option_vec_field_name4 = option_vec_field_name.clone();
    let option_vec_field_name5 = option_vec_field_name.clone();
    let option_vec_field_name6 = option_vec_field_name.clone();

    TokenStream::from(quote! {

        impl AoristConcept for #struct_name {
            fn get_child_concepts<'a, 'b>(&'a self) -> Vec<Concept<'b>> where 'a : 'b {
                let mut concepts = vec![
                    #(
                      Concept::#bare_field_type(&self.#bare_field_name7),
                    )*
                ];
                #(
                    for x in &self.#vec_field_name6 {
                        concepts.push(Concept::#vec_field_type(&x));
                    }
                )*
                #(
                    if let Some(ref v) = self.#option_vec_field_name6 {
                        for x in v {
                            concepts.push(Concept::#option_vec_field_type(&x));
                        }
                    }
                )*
                concepts
            }
            fn get_tag(&self) -> Option<String> {
                self.tag.clone()
            }
            fn traverse_constrainable_children(
                &self,
                mut upstream_constraints: Vec<Arc<RwLock<Constraint>>>
            ) {


                // ingest upstream constraints, including those rooted at the
                // same level
                for focal_constraint in self.get_constraints().clone() {
                    let mut my_upstream_constraints = upstream_constraints.clone();
                    let my_uuid = focal_constraint.read().unwrap().get_uuid();
                    // add my own constraints to upstream_constraints
                    for constraint in self.get_constraints().clone() {
                        if constraint.read().unwrap().get_uuid() != my_uuid {
                            my_upstream_constraints.push(constraint.clone());
                        }
                    }
                    focal_constraint.write().unwrap().ingest_upstream_constraints(my_upstream_constraints);
                }
                for constraint in self.get_constraints().clone() {
                    upstream_constraints.push(constraint.clone());
                }

                #(
                    self.#bare_field_name.traverse_constrainable_children(upstream_constraints.clone());
                )*
                #(
                    for x in &self.#vec_field_name {
                        x.traverse_constrainable_children(upstream_constraints.clone());
                    }
                )*
                #(
                    if let Some(ref v) = self.#option_vec_field_name {
                        for x in v {
                            x.traverse_constrainable_children(upstream_constraints.clone())
                        }
                    }
                )*
            }
            fn get_constraints(&self) -> &Vec<Arc<RwLock<Constraint>>> {
                &self.constraints
            }
            fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
                // TODO: this is where we should enforce deduplication
                let mut downstream: Vec<Arc<RwLock<Constraint>>> = Vec::new();
                for constraint in &self.constraints.clone() {
                    downstream.push(constraint.clone());
                    /*for elem in constraint.get_downstream_constraints() {
                        downstream.push(elem.clone());
                    }*/
                }
                #(
                    for constraint in &self.#bare_field_name6.get_downstream_constraints() {
                         downstream.push(constraint.clone());
                    }
                )*
                #(
                    for elem in &self.#vec_field_name5 {
                        for constraint in elem.get_downstream_constraints() {
                            downstream.push(constraint.clone());
                        }
                    }
                )*
                #(
                    if let Some(ref v) = self.#option_vec_field_name5 {
                        for elem in v {
                            for constraint in elem.get_downstream_constraints() {
                                downstream.push(constraint.clone());
                            }
                        }
                    }
                )*
                downstream
            }
            fn compute_constraints(&mut self) {
                let mut constraints: Vec<Arc<RwLock<Constraint>>> = Vec::new();
                #(
                    self.#bare_field_name3.compute_constraints();
                    for constraint in self.#bare_field_name2.get_downstream_constraints() {
                         constraints.push(constraint.clone());
                    }
                )*
                #(
                    for elem in self.#vec_field_name2.iter_mut() {
                        elem.compute_constraints();
                        for constraint in elem.get_downstream_constraints() {
                            constraints.push(constraint.clone());
                        }
                    }
                )*
                #(
                    if let Some(ref mut v) = self.#option_vec_field_name2 {
                        for elem in v.iter_mut() {
                            elem.compute_constraints();
                            for constraint in elem.get_downstream_constraints() {
                                constraints.push(constraint.clone());
                            }
                        }
                    }
                )*
                let new_constraints = vec![
                    #(
                        Arc::new(RwLock::new(Constraint{
                            name: stringify!(#constraint).to_string(),
                            root: stringify!(#struct_name).to_string(),
                            requires: None,
                            inner: Some(
                                AoristConstraint::#constraint(
                                    crate::constraint::#constraint::new(
                                        self.get_uuid(),
                                        constraints.clone(),
                                    )
                                )
                            ),
                        })),
                    )*
                ];
                for c in new_constraints.into_iter() {
                    self.constraints.push(c);
                }
                //println!("Computed {} constraints on {}.", self.constraints.len(),
                //stringify!(#struct_name));
            }
            fn get_uuid(&self) -> Uuid {
                if let Some(uuid) = self.uuid {
                    return uuid.clone();
                }
                panic!("Uuid was not set on object.");
            }
            fn get_children_uuid(&self) -> Vec<Uuid> {
                let mut uuids: Vec<Uuid> = Vec::new();
                #(
                    uuids.push(self.#bare_field_name4.get_uuid());
                )*
                #(
                    for elem in &self.#vec_field_name3 {
                        uuids.push(elem.get_uuid());
                    }
                )*
                #(
                    if let Some(ref v) = self.#option_vec_field_name3 {
                        for elem in v {
                            uuids.push(elem.get_uuid());
                        }
                    }
                )*
                uuids
            }
            fn compute_uuids(&mut self) {
                #(
                    self.#bare_field_name5.compute_uuids();
                )*
                #(
                    for elem in self.#vec_field_name4.iter_mut() {
                        elem.compute_uuids();
                    }
                )*
                #(
                    if let Some(ref mut v) = self.#option_vec_field_name4 {
                        for elem in v.iter_mut() {
                            elem.compute_uuids();
                        }
                    }
                )*
                self.uuid = Some(self.get_uuid_from_children_uuid());
            }
        }
    })
}
fn get_constraints_map(filename: String) -> HashMap<String, Vec<String>> {
    let raw_objects = read_file(&filename);
    let constraints = get_raw_objects_of_type(&raw_objects, "Constraint".into());
    let constraints_parsed: Vec<(String, String)> = constraints
        .into_iter()
        .map(|x| {
            (
                x.get("name").unwrap().as_str().unwrap().into(),
                x.get("root").unwrap().as_str().unwrap().into(),
            )
        })
        .collect();
    let mut constraints_map: HashMap<String, Vec<String>> = HashMap::new();
    for (name, root) in constraints_parsed {
        constraints_map.entry(root).or_insert(Vec::new()).push(name);
    }
    constraints_map
}

#[proc_macro_derive(Constrainable, attributes(constrainable))]
pub fn aorist_concept(input: TokenStream) -> TokenStream {
    // TODO: this should be passed somehow (maybe env var?)
    let constraints_map = get_constraints_map("basic.yaml".into());
    let input = parse_macro_input!(input as DeriveInput);
    //let constraint_names = AoristConstraint::get_required_constraint_names();
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => process_struct_fields(&fields.named, &input, &constraints_map),
        Data::Enum(DataEnum { variants, .. }) => {
            process_enum_variants(variants, &input, &constraints_map)
        }
        _ => panic!("expected a struct with named fields"),
    }
}

// Following: https://github.com/dtolnay/syn/issues/516
extern crate proc_macro;
use self::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::fs::OpenOptions;
use std::io::prelude::*;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, parse_quote, AttributeArgs, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, Meta, NestedMeta, Token, Type, Variant,
};
use type_macro_helpers::{extract_type_from_option, extract_type_from_vector};
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
    let variant = variants
        .iter()
        .map(|x| (x.ident.clone()))
        .collect::<Vec<_>>();

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

    for v in &variant {
        writeln!(file, "'{}'->'{}';", enum_name, v).unwrap();
    }

    TokenStream::from(quote! {
      impl AoristConcept for #enum_name {
        fn get_child_concepts<'a, 'b>(&'a self) -> Vec<Concept<'b>> where 'a : 'b {
          vec![
              match self {
                #(
                  #enum_name::#variant(x) => Concept::#variant(
                      (
                          &x,
                          0, Some((
                              self.get_uuid(),
                              stringify!(#enum_name).to_string()
                          ))
                      )
                   ),
                )*
              }
          ]
        }
        fn get_tag(&self) -> Option<String> {
            match self {
                #(
                  #enum_name::#variant(x) => x.get_tag(),
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
        fn add_constraints(
            &mut self,
            constraint_map: &std::collections::HashMap<Uuid, Vec<Arc<RwLock<Constraint>>>>
        ) {
            match constraint_map.get(&self.get_uuid()) {
                Some(enum_constraints) => match self {
                    #(
                        #enum_name::#variant(ref mut x) => {
                            for el in enum_constraints.iter() {
                                x.add_constraint(el.clone());
                            };
                            x.add_constraints(constraint_map);
                        },
                    )*
                },
                None => panic!(format!("Could not find constraints for {}",
                stringify!(#enum_name))),
            }
        }
        fn compute_constraints(&mut self) {
          let uuid = self.get_uuid();
          match self {
            #(
              #enum_name::#variant(ref mut x) => {
                  x.compute_constraints();
              }
            )*
          }
          let downstream = self.get_downstream_constraints();
          let mut enum_constraints = Vec::new();
          #(
            if crate::constraint::#constraint::should_add(&self) {
              enum_constraints.push(
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
              );
            }
          )*
          match self {
            #(
              #enum_name::#variant(ref mut x) => {
                  for el in enum_constraints.into_iter() {
                      x.add_constraint(el);
                  };
              }
            )*
          }
        }

        fn add_constraint(&mut self, constraint: Arc<RwLock<Constraint>>)  {
          match self {
            #(
              #enum_name::#variant(ref mut x) => x.add_constraint(constraint),
            )*
          }
        }

        fn get_constraints(
          &self,
        ) -> &Vec<Arc<RwLock<Constraint>>> {
          match self {
            #(
              #enum_name::#variant(x) => x.get_constraints(),
            )*
          }
        }

        fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
          match self {
            #(
              #enum_name::#variant(x) => x.get_downstream_constraints(),
            )*
          }
        }

        fn get_uuid(&self) -> Uuid {
          match self {
            #(
              #enum_name::#variant(x) => x.get_uuid(),
            )*
          }
        }
        fn get_children_uuid(&self) -> Vec<Uuid> {
          match self {
            #(
              #enum_name::#variant(x) => x.get_children_uuid(),
            )*
          }
        }
        fn compute_uuids(&mut self) {
          match self {
            #(
              #enum_name::#variant(x) => x.compute_uuids(),
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
    let bare_field_name = bare_field.clone().map(|x| x.0).collect::<Vec<_>>();
    let bare_field_type = bare_field.clone().map(|x| x.1).collect::<Vec<_>>();
    let vec_field_name = vec_field.clone().map(|x| x.0).collect::<Vec<_>>();
    let vec_field_type = vec_field.clone().map(|x| x.1).collect::<Vec<_>>();
    let option_vec_field_name = option_vec_field.clone().map(|x| x.0).collect::<Vec<_>>();
    let option_vec_field_type = option_vec_field.clone().map(|x| x.1).collect::<Vec<_>>();

    TokenStream::from(quote! {

        impl AoristConcept for #struct_name {
            fn get_child_concepts<'a, 'b>(&'a self) -> Vec<Concept<'b>> where 'a : 'b {
                let id = Some((
                    self.get_uuid(),
                    stringify!(#struct_name).to_string()
                ));
                let mut concepts = vec![
                    #(
                      Concept::#bare_field_type((
                          &self.#bare_field_name,
                          0,
                          id.clone()
                      )),
                    )*
                ];
                #(
                    for (i, x) in self.#vec_field_name.iter().enumerate() {
                        concepts.push(
                            Concept::#vec_field_type((&x, i + 1, id.clone()))
                        );
                    }
                )*
                #(
                    if let Some(ref v) = self.#option_vec_field_name {
                        for (i, x) in v.iter().enumerate() {
                            concepts.push(
                                Concept::#option_vec_field_type(
                                    (&x, i + 1, id.clone())
                                )
                            );
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
                mut upstream_constraints: Vec<Arc<RwLock<Constraint>>>,
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
                    for constraint in &self.#bare_field_name.get_downstream_constraints() {
                         downstream.push(constraint.clone());
                    }
                )*
                #(
                    for elem in &self.#vec_field_name {
                        for constraint in elem.get_downstream_constraints() {
                            downstream.push(constraint.clone());
                        }
                    }
                )*
                #(
                    if let Some(ref v) = self.#option_vec_field_name {
                        for elem in v {
                            for constraint in elem.get_downstream_constraints() {
                                downstream.push(constraint.clone());
                            }
                        }
                    }
                )*
                downstream
            }
            fn add_constraints(
                &mut self,
                constraint_map: &std::collections::HashMap<Uuid, Vec<Arc<RwLock<Constraint>>>>
            ) {
                match constraint_map.get(&self.get_uuid()) {
                    None => panic!(format!("Could not find constraints for {}",
                    stringify!(#struct_name))),
                    Some(constraints) => { 
                        for c in constraints.iter() {
                            self.constraints.push(c.clone());
                        }
                        #(
                            self.#bare_field_name.add_constraints(constraint_map);
                        )*
                        #(
                            for elem in self.#vec_field_name.iter_mut() {
                                elem.add_constraints(constraint_map);
                            }
                        )*
                        #(
                            if let Some(ref mut v) = self.#option_vec_field_name {
                                for elem in v.iter_mut() {
                                    elem.add_constraints(constraint_map);
                                }
                            }
                        )*
                    }
                }
            }
            fn compute_constraints(&mut self) {
                let mut constraints: Vec<Arc<RwLock<Constraint>>> = Vec::new();
                #(
                    self.#bare_field_name.compute_constraints();
                    for constraint in self.#bare_field_name.get_downstream_constraints() {
                         constraints.push(constraint.clone());
                    }
                )*
                #(
                    for elem in self.#vec_field_name.iter_mut() {
                        elem.compute_constraints();
                        for constraint in elem.get_downstream_constraints() {
                            constraints.push(constraint.clone());
                        }
                    }
                )*
                #(
                    if let Some(ref mut v) = self.#option_vec_field_name {
                        for elem in v.iter_mut() {
                            elem.compute_constraints();
                            for constraint in elem.get_downstream_constraints() {
                                constraints.push(constraint.clone());
                            }
                        }
                    }
                )*
                let mut new_constraints = Vec::new();

                #(
                  if crate::constraint::#constraint::should_add(&self) {
                    new_constraints.push(
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
                    );
                  }
                )*
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
                    uuids.push(self.#bare_field_name.get_uuid());
                )*
                #(
                    for elem in &self.#vec_field_name {
                        uuids.push(elem.get_uuid());
                    }
                )*
                #(
                    if let Some(ref v) = self.#option_vec_field_name {
                        for elem in v {
                            uuids.push(elem.get_uuid());
                        }
                    }
                )*
                uuids
            }
            fn add_constraint(&mut self, constraint: Arc<RwLock<Constraint>>)  {
                self.constraints.push(constraint);
            }
            fn compute_uuids(&mut self) {
                #(
                    self.#bare_field_name.compute_uuids();
                )*
                #(
                    for elem in self.#vec_field_name.iter_mut() {
                        elem.compute_uuids();
                    }
                )*
                #(
                    if let Some(ref mut v) = self.#option_vec_field_name {
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
pub fn constrainable(input: TokenStream) -> TokenStream {
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
        _ => panic!("expected a struct with named fields or an enum"),
    }
}

fn field_is_constrainable(field: &Field) -> bool {
    for a in &field.attrs {
        if let Ok(Meta::Path(x)) = a.parse_meta() {
            if x.is_ident("constrainable") {
                return true;
            }
        }
    }
    return false;
}

fn get_constrainable_fields(fields: Vec<Field>) -> (Vec<Field>, Vec<Field>) {
    let mut constrainable_fields: Vec<Field> = Vec::new();
    let mut unconstrainable_fields: Vec<Field> = Vec::new();
    for field in fields {
        if field_is_constrainable(&field) {
            constrainable_fields.push(field);
        } else {
            unconstrainable_fields.push(field);
        }
    }
    (constrainable_fields, unconstrainable_fields)
}

fn extract_names_and_types(fields: &Vec<Field>) -> (Vec<Ident>, Vec<Type>) {
    let mut names: Vec<Ident> = Vec::new();
    let mut types: Vec<Type> = Vec::new();
    for field in fields {
        names.push(field.ident.as_ref().unwrap().clone());
        types.push(field.ty.clone());
    }
    (names, types)
}

fn extract_type_variants(
    fields: &Vec<Field>,
) -> (
    Vec<Type>,
    Vec<Type>,
    Vec<Type>,
    Vec<Type>,
    Vec<Ident>,
    Vec<Ident>,
    Vec<Ident>,
    Vec<Ident>,
) {
    let mut bare_types: Vec<Type> = Vec::new();
    let mut vec_types: Vec<Type> = Vec::new();
    let mut option_vec_types: Vec<Type> = Vec::new();
    let mut option_types: Vec<Type> = Vec::new();

    let mut bare_idents: Vec<Ident> = Vec::new();
    let mut vec_idents: Vec<Ident> = Vec::new();
    let mut option_vec_idents: Vec<Ident> = Vec::new();
    let mut option_idents: Vec<Ident> = Vec::new();

    for field in fields {
        let tt = &field.ty;
        let ident = field.ident.as_ref().unwrap().clone();

        if let Some(vec_type) = extract_type_from_vector(tt) {
            vec_types.push(vec_type.clone());
            vec_idents.push(ident.clone());
        } else if let Some(option_type) = extract_type_from_option(tt) {
            if let Some(option_vec_type) = extract_type_from_vector(option_type) {
                option_vec_types.push(option_vec_type.clone());
                option_vec_idents.push(ident.clone());
            } else {
                option_types.push(option_type.clone());
                option_idents.push(ident.clone());
            }
        } else {
            bare_types.push(tt.clone());
            bare_idents.push(ident.clone());
        }
    }
    (
        bare_types,
        vec_types,
        option_types,
        option_vec_types,
        bare_idents,
        vec_idents,
        option_idents,
        option_vec_idents,
    )
}

#[proc_macro_derive(InnerObject, attributes(py_default))]
pub fn constrain_object(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match &ast.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let enum_name = &ast.ident;
            let variant = variants
                .iter()
                .map(|x| (x.ident.clone()))
                .collect::<Vec<_>>();
            let quoted = quote! { paste! {
                #[derive(Clone, FromPyObject)]
                pub enum [<Inner #enum_name>] {
                    #(#variant([<Inner #variant>])),*
                }
                impl From<[<Inner #enum_name>]> for #enum_name {
                    fn from(inner: [<Inner #enum_name>]) -> Self {
                        match inner {
                             #(
                                 [<Inner #enum_name>]::#variant(x) => Self::#variant(#variant::from(x)),
                             )*
                        }
                    }
                }
            }};
            return proc_macro::TokenStream::from(quoted);
        }
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let struct_name = &ast.ident;
            let fields_filtered = fields
                .named
                .clone()
                .into_iter()
                .filter(|x| {
                    let ident = x.ident.as_ref().unwrap().to_string();
                    !(ident == "tag" || ident == "uuid" || ident == "constraints")
                })
                .collect::<Vec<_>>();

            let (constrainable, unconstrainable) =
                get_constrainable_fields(fields_filtered.clone());
            let (
                bare_type,
                vec_type,
                option_type,
                option_vec_type,
                bare_ident,
                vec_ident,
                option_ident,
                option_vec_ident,
            ) = extract_type_variants(&constrainable);
            let (unconstrainable_name, unconstrainable_type) =
                extract_names_and_types(&unconstrainable);
            let py_class_name = format!("{}", struct_name);
            let fields_with_default = fields_filtered
                .clone()
                .into_iter()
                .map(|x| {
                    let mut it = x
                        .attrs
                        .iter()
                        .map(|attr| {
                            let meta = attr.parse_meta().unwrap();
                            if let syn::Meta::NameValue(ref nv) = meta {
                                if nv.path.is_ident("py_default") {
                                    if let syn::Lit::Str(_) = nv.lit {
                                        let field_name = x.ident.as_ref().unwrap().clone();
                                        return Some(syn::MetaNameValue {
                                            path: syn::Path::from(field_name),
                                            eq_token: nv.eq_token.clone(),
                                            lit: nv.lit.clone(),
                                        });
                                    } else {
                                        panic!("py_default values should only be strings");
                                    }
                                }
                            }
                            None
                        })
                        .filter(|x| x.is_some());
                    let default_val = it.next();
                    if let Some(x) = default_val {
                        assert!(it.next().is_none());
                        return x;
                    }
                    None
                })
                .filter(|x| x.is_some())
                .map(|x| syn::NestedMeta::Meta(syn::Meta::NameValue(x.unwrap())))
                .collect::<syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>>();
            return TokenStream::from(quote! { paste! {

                #[pyclass(name=#py_class_name)]
                #[derive(Clone)]
                pub struct [<Inner #struct_name>] {
                    #(pub #bare_ident: [<Inner #bare_type>] ,)*
                    #(pub #vec_ident: Vec<[<Inner #vec_type>]> ,)*
                    #(pub #option_ident: Option<[<Inner #option_type>]> ,)*
                    #(pub #option_vec_ident: Option<Vec<[<Inner #option_vec_type>]>> ,)*
                    #(pub #unconstrainable_name: #unconstrainable_type,)*
                    pub tag: Option<String>,
                }

                #[pymethods]
                impl [<Inner #struct_name>] {
                    #[new]
                    #[args(#fields_with_default)]
                    fn new(
                        #(#bare_ident: [<Inner #bare_type>] ,)*
                        #(#vec_ident: Vec<[<Inner #vec_type>]> ,)*
                        #(#option_ident: Option<[<Inner #option_type>]> ,)*
                        #(#option_vec_ident: Option<Vec<[<Inner #option_vec_type>]>> ,)*
                        #(#unconstrainable_name: #unconstrainable_type,)*
                        tag: Option<String>,
                    ) -> Self {
                        Self {
                            #(#bare_ident,)*
                            #(#vec_ident,)*
                            #(#option_ident,)*
                            #(#option_vec_ident,)*
                            #(#unconstrainable_name,)*
                            tag
                        }
                    }
                }

                impl From<[<Inner #struct_name>]> for #struct_name {
                    fn from(inner: [<Inner #struct_name>]) -> Self {
                        Self {
                            #(#bare_ident: #bare_type::from(inner.#bare_ident),)*
                            #(#vec_ident: inner.#vec_ident.into_iter().map(|x| #vec_type::from(x)).collect(),)*
                            #(
                                #option_ident: match inner.#option_ident {
                                    None => None,
                                    Some(x) => Some(#option_type::from(x)),
                                },
                            )*
                            #(
                                #option_vec_ident: match inner.#option_vec_ident {
                                    None => None,
                                    Some(v) => Some(v.into_iter().map(|x| #option_vec_type::from(x)).collect()),
                                },
                            )*
                            #(#unconstrainable_name: inner.#unconstrainable_name,)*
                            uuid: None,
                            tag: inner.tag,
                            constraints: Vec::new(),
                        }
                    }
                }

            }});
        }

        _ => panic!("expected a struct with named fields or an enum"),
    }
}

fn get_derives(attrs: Vec<NestedMeta>) -> (Vec<NestedMeta>, Vec<NestedMeta>) {
    let mut derivatives: Vec<NestedMeta> = Vec::new();
    let mut derives: Vec<NestedMeta> = Vec::new();
    for attr in attrs {
        if let NestedMeta::Meta(Meta::List(x)) = attr {
            if x.path.is_ident("derivative") {
                derivatives = x.nested.into_iter().collect();
            } else if x.path.is_ident("derive") {
                derives = x.nested.into_iter().collect();
            } else {
                panic!("An attribute other than derive or derivative was specified");
            }
        } else {
            panic!("An attribute other than a MetaList was specified.");
        }
    }
    (derives, derivatives)
}

#[proc_macro_attribute]
pub fn aorist_concept(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_attrs = parse_macro_input!(args as AttributeArgs);
    let (extra_derives, extra_derivatives) = get_derives(input_attrs);

    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                Fields::Named(fields) => {
                    fields.named.push(
                        Field::parse_named
                            .parse2(quote! {
                            pub uuid: Option<Uuid>
                            })
                            .unwrap(),
                    );
                    fields.named.push(
                        Field::parse_named
                            .parse2(quote! {
                            pub tag: Option<String>
                            })
                            .unwrap(),
                    );
                    fields.named
                        .push(
                        Field::parse_named.parse2(quote! {
                            #[serde(skip)]
                            #[derivative(PartialEq = "ignore", Debug = "ignore", Hash = "ignore")]
                            pub constraints: Vec<Arc<RwLock<Constraint>>>
                        }).unwrap()
                    );
                }
                _ => (),
            }
            let quoted = quote! {
                #[derive(Derivative, Serialize, Deserialize, Constrainable, Clone, InnerObject#(, #extra_derives)*)]
                #[derivative(PartialEq, Debug, Eq)]
                #ast
            };
            let mut final_ast: DeriveInput = syn::parse2(quoted).unwrap();

            let (attr, mut derivatives) = final_ast
                .attrs
                .iter_mut()
                .filter(|attr| attr.path.is_ident("derivative"))
                .filter_map(|attr| match attr.parse_meta() {
                    Ok(Meta::List(meta_list)) => Some((attr, meta_list)),
                    _ => None, // kcov-ignore
                })
                .next()
                .unwrap();
            derivatives
                .nested
                .extend::<Punctuated<NestedMeta, Token![,]>>(
                    extra_derivatives.into_iter().collect(),
                );
            *attr = parse_quote!(#[#derivatives]);

            let quoted2 = quote! { #final_ast };
            return proc_macro::TokenStream::from(quoted2);
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let enum_name = &ast.ident;
            let variant = variants.iter().map(|x| (&x.ident));
            let quoted = quote! {
                #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, InnerObject)]
                #[serde(tag = "type")]
                pub enum #enum_name {
                    #(#variant(#variant)),*
                }
            };
            let mut final_ast: DeriveInput = syn::parse2(quoted).unwrap();
            let (attr, mut current_derives) = final_ast
                .attrs
                .iter_mut()
                .filter(|attr| attr.path.is_ident("derive"))
                .filter_map(|attr| match attr.parse_meta() {
                    Ok(Meta::List(meta_list)) => Some((attr, meta_list)),
                    _ => None, // kcov-ignore
                })
                .next()
                .unwrap();
            current_derives
                .nested
                .extend::<Punctuated<NestedMeta, Token![,]>>(
                    extra_derivatives.into_iter().collect(),
                );
            *attr = parse_quote!(#[#current_derives]);

            let quoted2 = quote! { #final_ast };

            return proc_macro::TokenStream::from(quoted2);
        }
        _ => panic!("expected a struct with named fields or an enum"),
    }
}

extern crate proc_macro;
use self::proc_macro::TokenStream;
use crate::builder::Builder;
use proc_macro2::Ident;
use quote::quote;
use std::fs::OpenOptions;
use std::io::prelude::*;
use syn::{Field, FieldsNamed, Meta, Type};
use type_macro_helpers::{
    extract_type_from_map, extract_type_from_option, extract_type_from_vector,
};
mod keyword {
    syn::custom_keyword!(path);
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

pub fn get_constrainable_fields(fields: Vec<Field>) -> (Vec<Field>, Vec<Field>) {
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

pub struct StructBuilder {
    pub bare_types: Vec<Type>,
    pub vec_types: Vec<Type>,
    pub option_vec_types: Vec<Type>,
    pub option_types: Vec<Type>,
    pub map_key_types: Vec<Type>,
    pub map_value_types: Vec<Type>,
    pub bare_idents: Vec<Ident>,
    pub vec_idents: Vec<Ident>,
    pub option_vec_idents: Vec<Ident>,
    pub option_idents: Vec<Ident>,
    pub map_idents: Vec<Ident>,
    pub fields_with_default: syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
    pub unconstrainable: Vec<Field>,
}

impl Builder for StructBuilder {
    type TInput = FieldsNamed;
    fn new(fields: &FieldsNamed) -> StructBuilder {
        let fields_filtered = fields
            .named
            .clone()
            .into_iter()
            .filter(|x| {
                let ident = x.ident.as_ref().unwrap().to_string();
                !(ident == "tag" || ident == "uuid" || ident == "constraints")
            })
            .collect::<Vec<_>>();
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

        let (constrainable, unconstrainable) = get_constrainable_fields(fields_filtered.clone());

        let mut bare_types: Vec<Type> = Vec::new();
        let mut vec_types: Vec<Type> = Vec::new();
        let mut option_vec_types: Vec<Type> = Vec::new();
        let mut option_types: Vec<Type> = Vec::new();
        let mut map_key_types: Vec<Type> = Vec::new();
        let mut map_value_types: Vec<Type> = Vec::new();

        let mut bare_idents: Vec<Ident> = Vec::new();
        let mut vec_idents: Vec<Ident> = Vec::new();
        let mut option_vec_idents: Vec<Ident> = Vec::new();
        let mut option_idents: Vec<Ident> = Vec::new();
        let mut map_idents: Vec<Ident> = Vec::new();

        for field in constrainable {
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
            } else if let Some((map_key_type, map_value_type)) = extract_type_from_map(tt) {
                map_key_types.push(map_key_type.clone());
                map_value_types.push(map_value_type.clone());
                map_idents.push(ident.clone());
            } else {
                bare_types.push(tt.clone());
                bare_idents.push(ident.clone());
            }
        }
        Self {
            bare_types,
            vec_types,
            option_types,
            option_vec_types,
            map_key_types,
            map_value_types,
            bare_idents,
            vec_idents,
            option_idents,
            option_vec_idents,
            map_idents,
            fields_with_default,
            unconstrainable,
        }
    }
    fn to_file(&self, struct_name: &Ident, file_name: &str) {
        let types = self
            .bare_idents
            .iter()
            .chain(self.option_vec_idents.iter())
            .chain(self.option_idents.iter())
            .chain(self.vec_idents.iter())
            .zip(
                self.bare_types
                    .iter()
                    .chain(self.option_vec_types.iter())
                    .chain(self.option_types.iter())
                    .chain(self.vec_types.iter()),
            );
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
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
                struct_name, type_val, ident,
            )
            .unwrap();
        }
    }
    fn to_concept_children_token_stream(&self, struct_name: &Ident) -> TokenStream {
        let (
            bare_type,
            vec_type,
            option_type,
            option_vec_type,
            map_value_type,
            bare_ident,
            vec_ident,
            option_ident,
            option_vec_ident,
            map_ident,
        ) = (
            &self.bare_types,
            &self.vec_types,
            &self.option_types,
            &self.option_vec_types,
            &self.map_value_types,
            &self.bare_idents,
            &self.vec_idents,
            &self.option_idents,
            &self.option_vec_idents,
            &self.map_idents,
        );

        TokenStream::from(quote! {

            impl AoristConceptChildren for #struct_name {
                fn get_child_concepts<'a, 'b>(&'a self) -> Vec<Concept<'b>> where 'a : 'b {
                    let id = Some((
                        self.get_uuid(),
                        stringify!(#struct_name).to_string()
                    ));
                    let mut concepts = vec![
                        #(
                          Concept::#bare_type((
                              &self.#bare_ident,
                              0,
                              id.clone()
                          )),
                        )*
                    ];
                    #(
                        if let Some(ref c) = self.#option_ident {
                            concepts.push(
                                Concept::#option_type((c, 0, id.clone()))
                            );
                        }
                    )*
                    #(
                        for (i, x) in self.#vec_ident.iter().enumerate() {
                            concepts.push(
                                Concept::#vec_type((&x, i + 1, id.clone()))
                            );
                        }
                    )*
                    #(
                        if let Some(ref v) = self.#option_vec_ident {
                            for (i, x) in v.iter().enumerate() {
                                concepts.push(
                                    Concept::#option_vec_type(
                                        (&x, i + 1, id.clone())
                                    )
                                );
                            }
                        }
                    )*
                    #(
                        let mut i = 0;
                        for v in self.#map_ident.values() {
                            concepts.push(
                                Concept::#map_value_type((&v, i + 1, id.clone()))
                            );
                            i += 1;
                        }
                    )*
                    concepts
                }
            }
        })
    }
    fn to_concept_token_stream(&self, struct_name: &Ident) -> TokenStream {
        let (
            bare_ident,
            vec_ident,
            option_ident,
            option_vec_ident,
            map_ident,
        ) = (
            &self.bare_idents,
            &self.vec_idents,
            &self.option_idents,
            &self.option_vec_idents,
            &self.map_idents,
        );

        TokenStream::from(quote! {

            impl AoristConcept for #struct_name {
                fn get_tag(&self) -> Option<String> {
                    self.tag.clone()
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
                        uuids.push(self.#bare_ident.get_uuid());
                    )*
                    #(
                        if let Some(ref c) = self.#option_ident {
                            uuids.push(c.get_uuid());
                        }
                    )*
                    #(
                        for elem in &self.#vec_ident {
                            uuids.push(elem.get_uuid());
                        }
                    )*
                    #(
                        if let Some(ref v) = self.#option_vec_ident {
                            for elem in v {
                                uuids.push(elem.get_uuid());
                            }
                        }
                    )*
                    #(
                        for elem in self.#map_ident.values() {
                            uuids.push(elem.get_uuid());
                        }
                    )*
                    uuids
                }
                fn compute_uuids(&mut self) {
                    #(
                        self.#bare_ident.compute_uuids();
                    )*
                    #(
                        if let Some(ref mut c) = self.#option_ident {
                            c.compute_uuids();
                        }
                    )*
                    #(
                        for elem in self.#vec_ident.iter_mut() {
                            elem.compute_uuids();
                        }
                    )*
                    #(
                        if let Some(ref mut v) = self.#option_vec_ident {
                            for elem in v.iter_mut() {
                                elem.compute_uuids();
                            }
                        }
                    )*
                    #(
                        for elem in self.#map_ident.values_mut() {
                            elem.compute_uuids();
                        }
                    )*
                    self.uuid = Some(self.get_uuid_from_children_uuid());
                }
            }
        })
    }
    fn to_python_token_stream(&self, struct_name: &Ident) -> TokenStream {
        let (
            bare_type,
            vec_type,
            option_type,
            option_vec_type,
            map_key_type,
            map_value_type,
            bare_ident,
            vec_ident,
            option_ident,
            option_vec_ident,
            map_ident,
            fields_with_default,
            unconstrainable,
        ) = (
            &self.bare_types,
            &self.vec_types,
            &self.option_types,
            &self.option_vec_types,
            &self.map_key_types,
            &self.map_value_types,
            &self.bare_idents,
            &self.vec_idents,
            &self.option_idents,
            &self.option_vec_idents,
            &self.map_idents,
            &self.fields_with_default,
            &self.unconstrainable,
        );
        let (unconstrainable_name, unconstrainable_type) =
            extract_names_and_types(&unconstrainable);

        let py_class_name = format!("{}", struct_name);
        TokenStream::from(quote! { paste! {

            #[pyclass(name=#py_class_name)]
            #[derive(Clone, PartialEq)]
            pub struct [<Inner #struct_name>] {
                #(pub #bare_ident: [<Inner #bare_type>] ,)*
                #(pub #vec_ident: Vec<[<Inner #vec_type>]> ,)*
                #(pub #option_ident: Option<[<Inner #option_type>]> ,)*
                #(pub #option_vec_ident: Option<Vec<[<Inner #option_vec_type>]>> ,)*
                #(
                  pub #map_ident: std::collections::BTreeMap<
                    #map_key_type,
                    [<Inner #map_value_type>]
                  >,
                )*
                #(pub #unconstrainable_name: #unconstrainable_type,)*
                pub tag: Option<String>,
            }

            #[pymethods]
            impl [<Inner #struct_name>] {
                #[new]
                #[args(#fields_with_default)]
                pub fn new(
                    #(#bare_ident: [<Inner #bare_type>] ,)*
                    #(#vec_ident: Vec<[<Inner #vec_type>]> ,)*
                    #(#option_ident: Option<[<Inner #option_type>]> ,)*
                    #(#option_vec_ident: Option<Vec<[<Inner #option_vec_type>]>> ,)*
                    #(
                      #map_ident: std::collections::BTreeMap<
                        #map_key_type, [<Inner #map_value_type>]
                      >,
                    )*
                    #(#unconstrainable_name: #unconstrainable_type,)*
                    tag: Option<String>,
                ) -> Self {
                    Self {
                        #(#bare_ident,)*
                        #(#vec_ident,)*
                        #(#option_ident,)*
                        #(#option_vec_ident,)*
                        #(#map_ident,)*
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
                        #(#map_ident: inner.#map_ident.into_iter().map(
                          |(k, v)| (
                            k.clone(),
                            #map_value_type::from(v)
                          )
                        ).collect(),)*
                        #(#unconstrainable_name: inner.#unconstrainable_name,)*
                        uuid: None,
                        tag: inner.tag,
                        constraints: Vec::new(),
                    }
                }
            }

            impl From<#struct_name> for [<Inner #struct_name>] {
                fn from(outer: #struct_name) -> Self {
                    Self {
                        #(#bare_ident: [<Inner #bare_type>]::from(outer.#bare_ident),)*
                        #(#vec_ident: outer.#vec_ident.into_iter().map(|x| [<Inner #vec_type>]::from(x)).collect(),)*
                        #(
                            #option_ident: match outer.#option_ident {
                                None => None,
                                Some(x) => Some([<Inner #option_type>]::from(x)),
                            },
                        )*
                        #(
                            #option_vec_ident: match outer.#option_vec_ident {
                                None => None,
                                Some(v) => Some(v.into_iter().map(|x| [<Inner #option_vec_type>]::from(x)).collect()),
                            },
                        )*
                        #(
                            #map_ident: outer.#map_ident.into_iter().map(
                                |(k, v)| (
                                    k.clone(),
                                    [<Inner #map_value_type>]::from(v)
                                )
                            ).collect(),
                        )*
                        #(#unconstrainable_name: outer.#unconstrainable_name,)*
                        tag: outer.tag,
                    }
                }
            }

        }})
    }
    fn to_base_token_stream(&self, struct_name: &Ident) -> TokenStream {
        let (
            bare_type,
            vec_type,
            option_type,
            option_vec_type,
            map_key_type,
            map_value_type,
            bare_ident,
            vec_ident,
            option_ident,
            option_vec_ident,
            map_ident,
            unconstrainable,
        ) = (
            &self.bare_types,
            &self.vec_types,
            &self.option_types,
            &self.option_vec_types,
            &self.map_key_types,
            &self.map_value_types,
            &self.bare_idents,
            &self.vec_idents,
            &self.option_idents,
            &self.option_vec_idents,
            &self.map_idents,
            &self.unconstrainable,
        );
        let (unconstrainable_name, unconstrainable_type) =
            extract_names_and_types(&unconstrainable);

        TokenStream::from(quote! { paste! {

            #[derive(Clone, PartialEq)]
            pub struct [<Base #struct_name>] {
                #(pub #bare_ident: [<Base #bare_type>] ,)*
                #(pub #vec_ident: Vec<[<Base #vec_type>]> ,)*
                #(pub #option_ident: Option<[< #option_type>]> ,)*
                #(pub #option_vec_ident: Option<Vec<[<Base #option_vec_type>]>> ,)*
                #(
                  pub #map_ident: std::collections::BTreeMap<
                    #map_key_type,
                    [<Base #map_value_type>]
                  >,
                )*
                #(pub #unconstrainable_name: #unconstrainable_type,)*
            }

            impl [<Base #struct_name>] {
                #[new]
                pub fn new(
                    #(#bare_ident: [<Base #bare_type>] ,)*
                    #(#vec_ident: Vec<[<Base #vec_type>]> ,)*
                    #(#option_ident: Option<[<Base #option_type>]> ,)*
                    #(#option_vec_ident: Option<Vec<[<Base #option_vec_type>]>> ,)*
                    #(
                      #map_ident: std::collections::BTreeMap<
                        #map_key_type, [<Base #map_value_type>]
                      >,
                    )*
                    #(#unconstrainable_name: #unconstrainable_type,)*
                ) -> Self {
                    Self {
                        #(#bare_ident,)*
                        #(#vec_ident,)*
                        #(#option_ident,)*
                        #(#option_vec_ident,)*
                        #(#map_ident,)*
                        #(#unconstrainable_name,)*
                    }
                }
            }

            impl From<[<Base #struct_name>]> for #struct_name {
                fn from(inner: [<Base #struct_name>]) -> Self {
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
                        #(#map_ident: inner.#map_ident.into_iter().map(
                          |(k, v)| (
                            k.clone(),
                            #map_value_type::from(v)
                          )
                        ).collect(),)*
                        #(#unconstrainable_name: inner.#unconstrainable_name,)*
                        uuid: None,
                        tag: None,
                        constraints: Vec::new(),
                    }
                }
            }

            impl From<#struct_name> for [<Base #struct_name>] {
                fn from(outer: #struct_name) -> Self {
                    Self {
                        #(#bare_ident: [<Base #bare_type>]::from(outer.#bare_ident),)*
                        #(#vec_ident: outer.#vec_ident.into_iter().map(|x| [<Base #vec_type>]::from(x)).collect(),)*
                        #(
                            #option_ident: match outer.#option_ident {
                                None => None,
                                Some(x) => Some([<Base #option_type>]::from(x)),
                            },
                        )*
                        #(
                            #option_vec_ident: match outer.#option_vec_ident {
                                None => None,
                                Some(v) => Some(v.into_iter().map(|x| [<Base #option_vec_type>]::from(x)).collect()),
                            },
                        )*
                        #(
                            #map_ident: outer.#map_ident.into_iter().map(
                                |(k, v)| (
                                    k.clone(),
                                    [<Base #map_value_type>]::from(v)
                                )
                            ).collect(),
                        )*
                        #(#unconstrainable_name: outer.#unconstrainable_name,)*
                    }
                }
            }

        }})
    }
}

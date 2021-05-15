// Following: https://github.com/dtolnay/syn/issues/516
extern crate proc_macro;
mod type_variants;

use self::proc_macro::TokenStream;
use crate::type_variants::{get_constrainable_fields, TypeVariants};
use quote::quote;
use std::fs::OpenOptions;
use std::io::prelude::*;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, parse_quote, AttributeArgs, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, Meta, NestedMeta, Token, Variant, FieldsNamed,
};
mod keyword {
    syn::custom_keyword!(path);
}

fn process_enum_variants(
    variants: &Punctuated<Variant, Comma>,
    input: &DeriveInput,
) -> TokenStream {
    let enum_name = &input.ident;
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
fn process_struct_fields(fields: &FieldsNamed, input: &DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let tv = TypeVariants::new(fields);
    tv.to_file(struct_name, "constrainables.txt");
    tv.to_concept_token_stream(struct_name)
}
#[proc_macro_derive(Constrainable, attributes(constrainable))]
pub fn constrainable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => process_struct_fields(fields, &input),
        Data::Enum(DataEnum { variants, .. }) => process_enum_variants(&variants, &input),
        _ => panic!("expected a struct with named fields or an enum"),
    }
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
                #[derive(Clone, FromPyObject, PartialEq)]
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
                impl From<#enum_name> for [<Inner #enum_name>] {
                    fn from(outer: #enum_name) -> Self {
                        match outer {
                             #(
                                 #enum_name::#variant(x) => Self::#variant([<Inner #variant>]::from(x)),
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

            let (_constrainable, unconstrainable) =
                get_constrainable_fields(fields_filtered.clone());
            let tv = TypeVariants::new(&fields);
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
            tv.to_python_token_stream(struct_name, fields_with_default, unconstrainable)
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
            let variant = variants.iter().map(|x| (&x.ident)).collect::<Vec<_>>();
            let quoted = quote! {
                #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, InnerObject#(, #extra_derives)*)]
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

            let quoted2 = quote! { #final_ast
                impl #enum_name {
                    pub fn is_same_variant_in_enum_as(&self, other: &Self) -> bool {
                        match (self, other) {
                            #(
                                (#enum_name::#variant(_), #enum_name::#variant(_)) => true,
                            )*
                            _ => false,
                        }
                    }
                }
            };

            return proc_macro::TokenStream::from(quoted2);
        }
        _ => panic!("expected a struct with named fields or an enum"),
    }
}

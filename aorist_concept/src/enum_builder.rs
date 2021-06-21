extern crate proc_macro;
use self::proc_macro::TokenStream;
use crate::builder::Builder;
use proc_macro2::Ident;
use quote::quote;
use std::fs::OpenOptions;
use std::io::prelude::*;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Variant;
mod keyword {
    syn::custom_keyword!(path);
}

pub struct EnumBuilder {
    pub variant_idents: Vec<Ident>,
}
impl Builder for EnumBuilder {
    type TInput = syn::punctuated::Punctuated<Variant, Comma>;
    fn new(variants: &Punctuated<Variant, Comma>) -> Self {
        let variant_idents = variants
            .iter()
            .map(|x| (x.ident.clone()))
            .collect::<Vec<Ident>>();
        Self { variant_idents }
    }
    fn to_python_token_stream(&self, enum_name: &Ident) -> TokenStream {
        let variant = &self.variant_idents;
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
    fn to_file(&self, enum_name: &Ident, file_name: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
            .unwrap();
        writeln!(
            file,
            "node [shape = box, fillcolor=gray, style=filled, fontname = Helvetica] '{}';",
            enum_name
        )
        .unwrap();

        for v in &self.variant_idents {
            writeln!(file, "'{}'->'{}';", enum_name, v).unwrap();
        }
    }
    fn to_base_token_stream(&self, enum_name: &Ident) -> TokenStream {
        let variant = &self.variant_idents;
        let quoted = quote! { paste! {
            #[derive(Clone, FromPyObject, PartialEq)]
            pub enum [<Base #enum_name>] {
                #(#variant([<Base #variant>])),*
            }
            impl From<[<Base #enum_name>]> for #enum_name {
                fn from(inner: [<Base #enum_name>]) -> Self {
                    match inner {
                         #(
                             [<Base #enum_name>]::#variant(x) => Self::#variant(#variant::from(x)),
                         )*
                    }
                }
            }
            impl From<#enum_name> for [<Base #enum_name>] {
                fn from(outer: #enum_name) -> Self {
                    match outer {
                         #(
                             #enum_name::#variant(x) => Self::#variant([<Base #variant>]::from(x)),
                         )*
                    }
                }
            }
        }};
        return proc_macro::TokenStream::from(quoted);
    }
    fn to_concept_children_token_stream(&self, enum_name: &Ident) -> TokenStream {
        let variant = &self.variant_idents;
        TokenStream::from(quote! { paste! {
          impl <'a, T> std::convert::From<(
              // enum name
              &str,
              // field name
              Option<&str>,
              // ix
              Option<usize>,
              // uuid
              Option<Uuid>,
              // wrapped reference
              &'a #enum_name
          )> for crate::WrappedConcept<'a, T> where
          #(
              T: [<CanBe #variant>]<'a>,
          )* {
              fn from(
                  tpl: (
                      &str,
                      Option<&str>,
                      Option<usize>,
                      Option<Uuid>,
                      &'a #enum_name,
                  )
              ) -> Self {
                  let (name, field, ix, uuid, children_enum) = tpl;
                  match children_enum {
                      #(
                          #enum_name::#variant(x) => crate::WrappedConcept{
                              inner: T::[<construct_ #variant:snake:lower>](x, ix, Some((uuid.unwrap(), name.to_string()))),
                              _phantom_lt: std::marker::PhantomData,
                          },
                      )*
                      _ => panic!("_phantom arm should not be activated"),
                  }
              }
          }
        }})
    }
    fn to_concept_token_stream(&self, enum_name: &Ident) -> TokenStream {
        let variant = &self.variant_idents;
        proc_macro::TokenStream::from(quote! { paste! {
          impl <'a> ConceptEnum<'a> for &'a #enum_name {}
          pub trait [<CanBe #enum_name>]<'a> {
              fn [<construct_ #enum_name:snake:lower>] (
                  obj_ref: &'a #enum_name,
                  ix: Option<usize>,
                  id: Option<(Uuid, String)>
              ) -> Self;
          }
          impl <'a> AoristConcept<'a> for #enum_name {
            type TChildrenEnum = &'a #enum_name;
            fn get_children(&'a self) -> Vec<(
                // enum name
                &str,
                // field name
                Option<&str>,
                // ix
                Option<usize>,
                // uuid
                Option<Uuid>,
                &'a #enum_name
            )> {
                vec![(
                    stringify!(#enum_name),
                    None,
                    None,
                    Some(self.get_uuid()),
                    &self
                )]
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
        }})
    }
    fn to_python_token_stream_new(&self, enum_name: &Ident) -> TokenStream {
        let variant = &self.variant_idents;
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
    fn to_concept_token_stream2(&self, enum_name: &Ident) -> TokenStream {
        let variant = &self.variant_idents;
        proc_macro::TokenStream::from(quote! { paste! {
          //impl <'a> ConceptEnum<'a> for &'a #enum_name {}
          impl ConceptEnumNew for AoristRef<#enum_name> {}
          pub trait [<CanBe #enum_name>] {
              fn [<construct_ #enum_name:snake:lower>] (
                  obj_ref: AoristRef<#enum_name>,
                  ix: Option<usize>,
                  id: Option<(Uuid, String)>
              ) -> Self;
          }
          impl AoristConceptNew for AoristRef<#enum_name> {
              type TChildrenEnum = AoristRef<#enum_name>;
          }
          /*impl <'a> AoristConcept<'a> for #enum_name {
            type TChildrenEnum = &'a #enum_name;
            fn get_children(&'a self) -> Vec<(
                // enum name
                &str,
                // field name
                Option<&str>,
                // ix
                Option<usize>,
                // uuid
                Option<Uuid>,
                &'a #enum_name
            )> {
                vec![(
                    stringify!(#enum_name),
                    None,
                    None,
                    Some(self.get_uuid()),
                    &self
                )]
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
          }*/
        }})
    }
}

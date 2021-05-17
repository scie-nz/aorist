// Following: https://github.com/dtolnay/syn/issues/516
extern crate proc_macro;
mod enum_builder;
mod struct_builder;

use self::proc_macro::TokenStream;
use crate::enum_builder::EnumBuilder;
use crate::struct_builder::StructBuilder;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, parse_quote, AttributeArgs, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, FieldsNamed, Meta, NestedMeta, Token, Variant,
};
mod keyword {
    syn::custom_keyword!(path);
}

fn process_enum_variants(
    variants: &Punctuated<Variant, Comma>,
    input: &DeriveInput,
) -> TokenStream {
    let enum_name = &input.ident;
    let builder = EnumBuilder::new(variants);
    builder.to_file(enum_name, "constraints.txt");
    builder.to_concept_token_stream(enum_name)
}
fn process_struct_fields(fields: &FieldsNamed, input: &DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let tv = StructBuilder::new(fields);
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
            let builder = EnumBuilder::new(variants);
            let base_stream = builder.to_base_token_stream(enum_name);
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
            let base_stream = builder.to_base_token_stream(struct_name);
            let python_stream = builder.to_python_token_stream(struct_name);
            //base_stream.into_iter().chain(python_stream.into_iter()).collect()
            python_stream
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

fn add_aorist_fields(struct_data: &mut syn::DataStruct) {
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
            fields.named.push(
                Field::parse_named
                    .parse2(quote! {
                        #[serde(skip)]
                        #[derivative(PartialEq = "ignore", Debug = "ignore", Hash = "ignore")]
                        pub constraints: Vec<Arc<RwLock<Constraint>>>
                    })
                    .unwrap(),
            );
        }
        _ => (),
    }
}

fn extend_derivatives(ast: &mut DeriveInput, extra_derivatives: Vec<NestedMeta>) {
    let (attr, mut derivatives) = ast
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
        .extend::<Punctuated<NestedMeta, Token![,]>>(extra_derivatives.into_iter().collect());
    *attr = parse_quote!(#[#derivatives]);
}

#[proc_macro_attribute]
pub fn aorist_concept(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_attrs = parse_macro_input!(args as AttributeArgs);
    let (extra_derives, extra_derivatives) = get_derives(input_attrs);

    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            add_aorist_fields(struct_data);
            let quoted = quote! {
                #[derive(
                    Derivative, Serialize, Deserialize,
                    Constrainable, Clone, InnerObject#(, #extra_derives)*
                )]
                #[derivative(PartialEq, Debug, Eq)]
                #ast
            };
            let mut final_ast: DeriveInput = syn::parse2(quoted).unwrap();
            extend_derivatives(&mut final_ast, extra_derivatives);
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

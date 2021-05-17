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
use proc_macro2::Span;
use syn::token::Comma;
use syn::{
    parse_macro_input, parse_quote, AttributeArgs, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, FieldsNamed, Meta, NestedMeta, Token, Variant, LitStr, Path,
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

fn extend_metas(ast: &mut DeriveInput, extra_metas: Vec<NestedMeta>, ident: &str) {
    let (attr, mut metas) = ast
        .attrs
        .iter_mut()
        .filter(|attr| attr.path.is_ident(ident))
        .filter_map(|attr| match attr.parse_meta() {
            Ok(Meta::List(meta_list)) => Some((attr, meta_list)),
            _ => None, // kcov-ignore
        })
        .next()
        .unwrap();
    metas
        .nested
        .extend::<Punctuated<NestedMeta, Token![,]>>(extra_metas.into_iter().collect());
    *attr = parse_quote!(#[#metas]);
}
fn extend_derivatives(ast: &mut DeriveInput, extra_derivatives: Vec<NestedMeta>) {
    extend_metas(ast, extra_derivatives, "derivative");
}
fn extend_derives(ast: &mut DeriveInput, extra_derives: Vec<NestedMeta>) {
    extend_metas(ast, extra_derives, "derive");
}

#[proc_macro_attribute]
pub fn aorist_concept(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_attrs = parse_macro_input!(args as AttributeArgs);
    let (mut extra_derives, extra_derivatives) = get_derives(input_attrs);
    let path = LitStr::new("InnerObject", Span::call_site()).parse_with(Path::parse_mod_style).unwrap();
    let inner_object = NestedMeta::Meta(Meta::Path(
        path,
    ));
    extra_derives.push(inner_object);

    let mut ast = parse_macro_input!(input as DeriveInput);
    let quoted2 = match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            add_aorist_fields(struct_data);
            let quoted = quote! {
                #[derive(
                    Derivative, Serialize, Deserialize,
                    Constrainable, Clone,
                )]
                #[derivative(PartialEq, Debug, Eq)]
                #ast
            };
            let mut final_ast: DeriveInput = syn::parse2(quoted).unwrap();
            
            extend_derivatives(&mut final_ast, extra_derivatives);
            extend_derives(&mut final_ast, extra_derives);

            quote! { #final_ast }
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let enum_name = &ast.ident;
            let variant = variants.iter().map(|x| (&x.ident)).collect::<Vec<_>>();
            let quoted = quote! {
                #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, )]
                #[serde(tag = "type")]
                pub enum #enum_name {
                    #(#variant(#variant)),*
                }
            };
            let mut final_ast: DeriveInput = syn::parse2(quoted).unwrap();
            extend_derives(&mut final_ast, extra_derives);
            extend_derives(&mut final_ast, extra_derivatives);

            quote! {
                #final_ast
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
            }
        }
        _ => panic!("expected a struct with named fields or an enum"),
    };
    return proc_macro::TokenStream::from(quoted2);
}

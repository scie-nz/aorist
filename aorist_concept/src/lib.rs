// Following: https://github.com/dtolnay/syn/issues/516
extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Meta};

mod keyword {
    syn::custom_keyword!(path);
}
#[proc_macro_derive(Constrainable, attributes(constrainable))]
pub fn aorist_concept(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().filter(|field| field.attrs.iter().filter(|a| {
        match a.parse_meta() {
            Ok(Meta::Path(x)) => x.is_ident("constrainable"),
            _ => false
        }
    }).collect::<Vec<_>>().len() > 0).map(|field| &field.ident);
    let _field_type = fields.iter().map(|field| &field.ty);

    let struct_name = &input.ident;

    TokenStream::from(quote! {

        impl AoristConcept for #struct_name {
            fn traverse_constrainable_children(&self) {
                #(
                    self.#field_name.traverse_constrainable_children();
                )*
            }
        }
    })
}

// Following: https://github.com/dtolnay/syn/issues/516
extern crate proc_macro;
mod builder;
mod builder_enum;
mod concept_builder;
mod enum_builder;
mod struct_builder;

use self::proc_macro::TokenStream;
use crate::builder_enum::BuilderEnum;
use crate::concept_builder::{RawConceptBuilder, TConceptBuilder};
use syn::{parse_macro_input, AttributeArgs, DeriveInput};
mod keyword {
    syn::custom_keyword!(path);
}

#[proc_macro_attribute]
pub fn aorist(args: TokenStream, input: TokenStream) -> TokenStream {
    let builder = match RawConceptBuilder::new(vec![
        "Constrainable",
        "aorist_concept::ConstrainableWithChildren",
    ].into_iter().collect()) {
        Ok(x) => x,
        Err(err) => panic!("Cannot create RawConceptBuilder: {:?}", err),
    };
    let input_attrs = parse_macro_input!(args as AttributeArgs);
    let ast = parse_macro_input!(input as DeriveInput);
    match builder.gen(input_attrs.into_iter().collect(), ast) {
        Ok(x) => x,
        Err(err) => panic!("Cannot apply #[aorist] macro: {:?}", err),
    }
}
#[proc_macro_derive(Constrainable, attributes(constrainable))]
pub fn constrainable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let builder = BuilderEnum::new(input);
    //builder.to_file(name, "constraints.txt");
    match builder.to_concept_token_stream() {
        Ok(x) => x,
        Err(err) => panic!(
            "Cannot render Constrainable for {}: {:?}",
            builder.get_name(),
            err
        ),
    }
}
#[proc_macro_derive(ConstrainableWithChildren)]
pub fn constrainable_with_children(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let builder = BuilderEnum::new(input);
    match builder.to_concept_children_token_stream() {
        Ok(x) => x,
        Err(err) => panic!(
            "Cannot render ConstrainableWithChildren for {}: {:?}",
            builder.get_name(),
            err
        ),
    }
}

use quote::{quote, ToTokens};
use syn::{Data, DataStruct, DeriveInput, Fields};
use type_macro_helpers::{extract_type_from_vector, extract_type_path};

#[cfg(test)]
#[test] 
fn test_something() {
    let t = quote!(struct MyStruct {a: Vec<String>}).into_token_stream();
    let ast: DeriveInput = syn::parse2(t).unwrap();
    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = ast.data {
        assert_eq!(fields.named.len(), 1);
        let field =
        fields.named.iter().collect::<Vec<_>>().get(0).unwrap().clone();
        let tp = extract_type_path(&field.ty).unwrap();
        let idents_of_path = tp
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        assert_eq!(idents_of_path.len(), 4);
        assert_eq!(idents_of_path, "Vec|");
        assert!(extract_type_from_vector(&field.ty).is_some());
    }
}

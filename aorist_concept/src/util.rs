use syn::punctuated::Pair;
use syn::token::Colon2;
use syn::{GenericArgument, Path, PathArguments, PathSegment};
use aorist_error::{AResult, AoristError};

pub fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
    match *ty {
        syn::Type::Group(ref typegroup) => extract_type_path(&typegroup.elem),
        syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
        _ => None,
    }
}

fn get_inner_type<'a>(
    ty: &'a syn::Type,
    idents: Vec<String>,
) -> Option<Pair<&'a PathSegment, &'a Colon2>> {
    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    let extract_option_segment = |path: &'a Path| -> Option<Pair<&'a PathSegment, &'a Colon2>> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        idents
            .into_iter()
            .find(|s| idents_of_path == *s)
            .and_then(|_| path.segments.last())
            .map(Pair::End)
    };

    let tp = extract_type_path(&ty);
    tp.and_then(|path| extract_option_segment(path))
}

// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn/55277337
// https://rust-syndication.github.io/rss/src/derive_builder_core/setter.rs.html#198
fn extract_inner_from_bracketed_type<'a>(
    ty: &'a syn::Type,
    idents: Vec<String>,
) -> AResult<Option<&'a syn::Type>> {
    let tp2 = get_inner_type(ty, idents);
    let tp3 = tp2.and_then(|pair_path_segment| {
        let type_params = &pair_path_segment.into_value().arguments;
        // It should have only on angle-bracketed param ("<String>"):
        match *type_params {
            PathArguments::AngleBracketed(ref params) => params.args.first(),
            _ => None,
        }
    });
    Ok(tp3.and_then(|generic_arg| match *generic_arg {
        GenericArgument::Type(ref ty) => Some(ty),
        _ => None,
    }))
}

fn extract_inner_from_double_bracketed_type<'a>(
    ty: &'a syn::Type,
    idents: Vec<String>,
) -> AResult<Option<(&'a syn::Type, &'a syn::Type)>> {
    let tp2 = get_inner_type(ty, idents);
    if let Some(pair_path_segment) = tp2 {
        let type_params = &pair_path_segment.into_value().arguments;
        // It should have only on angle-bracketed param ("<String>"):
        if let PathArguments::AngleBracketed(ref params) = type_params {
            assert_eq!(params.args.len(), 2);
            let mut it = params.args.iter();
            if let Some(ref first) = it.next() {
                if let GenericArgument::Type(ref ty1) = first {
                    if let Some(second) = it.next() {
                        if let GenericArgument::Type(ref ty2) = second {
                            return Ok(Some((ty1, ty2)));
                        }
                        return Err(AoristError::OtherError(format!(
                            "2nd argument ({:?}) should be a Type.",
                            second
                        )));
                    }
                    return Err(AoristError::OtherError(
                        "Could not parse 2nd argument.".into(),
                    ));
                }
                return Err(AoristError::OtherError(format!(
                    "1st argument ({:?}) should be a Type.",
                    first
                )));
            };
            return Err(AoristError::OtherError(
                "Could not parse 1st argument.".into(),
            ));
        }
    }
    Ok(None)
}

pub fn extract_type_from_option(ty: &syn::Type) -> AResult<Option<&syn::Type>> {
    extract_inner_from_bracketed_type(
        ty,
        vec![
            "AOption|".to_string(),
            "aorist_primitives|AOption|".to_string(),
            "aorist_primitives|AOption".to_string(),
        ]
        .into_iter()
        .collect(),
    )
}

pub fn extract_type_from_vector(ty: &syn::Type) -> AResult<Option<&syn::Type>> {
    extract_inner_from_bracketed_type(
        ty,
        vec![
            "aorist_primitives|AVec".to_string(),
            "AVec|".to_string(),
            "aorist_primitives|AVec|".to_string(),
        ]
        .into_iter()
        .collect(),
    )
}

pub fn extract_type_from_map(ty: &syn::Type) -> AResult<Option<(&syn::Type, &syn::Type)>> {
    extract_inner_from_double_bracketed_type(
        ty,
        vec!["BTreeMap|".to_string(), "std|collections|BTreeMap|".into()]
            .into_iter()
            .collect(),
    )
}

pub fn extract_type_from_aorist_ref(ty: &syn::Type) -> AResult<Option<&syn::Type>> {
    extract_inner_from_bracketed_type(
        ty,
        vec!["RArc|".to_string(), "AoristRef|".to_string()]
            .into_iter()
            .collect(),
    )
}

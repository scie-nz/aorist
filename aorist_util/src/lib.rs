use serde_yaml::{from_str, Value};
use std::collections::HashMap;
use std::fs;
use syn::punctuated::Pair;
use syn::token::Colon2;
use syn::{GenericArgument, Path, PathArguments, PathSegment};
use tracing;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn read_file(filename: &str) -> Vec<HashMap<String, Value>> {
    let s = fs::read_to_string(filename).unwrap();
    s.split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| from_str(x).unwrap())
        .collect()
}
pub fn get_raw_objects_of_type(
    raw_objects: &Vec<HashMap<String, Value>>,
    object_type: String,
) -> Vec<HashMap<String, Value>> {
    raw_objects
        .into_iter()
        .filter(|x| x.get("type").unwrap().as_str().unwrap() == object_type)
        .map(|x| {
            x.get("spec")
                .unwrap()
                .as_mapping()
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k.as_str().unwrap().into(), v.clone()))
                .collect()
        })
        .collect::<Vec<HashMap<String, Value>>>()
}
/// Configures logging for Aorist based on a LOG_LEVEL envvar.
/// Valid values are: error/warn/info/debug/trace/off (default: info)
pub fn init_logging() {
    let filter_layer = EnvFilter::try_from_env("LOG_LEVEL")
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("Failed to initialize filter layer");

    tracing::subscriber::set_global_default(
        tracing_subscriber::Registry::default()
            .with(filter_layer)
            .with(fmt::layer()),
    )
    .expect("Failed to set default subscriber");
}

pub fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
    match *ty {
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
) -> Option<&'a syn::Type> {
    let tp2 = get_inner_type(ty, idents);
    let tp3 = tp2.and_then(|pair_path_segment| {
        let type_params = &pair_path_segment.into_value().arguments;
        // It should have only on angle-bracketed param ("<String>"):
        match *type_params {
            PathArguments::AngleBracketed(ref params) => params.args.first(),
            _ => None,
        }
    });
    tp3.and_then(|generic_arg| match *generic_arg {
        GenericArgument::Type(ref ty) => Some(ty),
        _ => None,
    })
}

fn extract_inner_from_double_bracketed_type<'a>(
    ty: &'a syn::Type,
    idents: Vec<String>,
) -> Option<(&'a syn::Type, &'a syn::Type)> {
    let tp2 = get_inner_type(ty, idents);
    let tp3 = tp2.and_then(|pair_path_segment| {
        let type_params = &pair_path_segment.into_value().arguments;
        // It should have only on angle-bracketed param ("<String>"):
        match *type_params {
            PathArguments::AngleBracketed(ref params) => {
                assert_eq!(params.args.len(), 2);
                let mut it = params.args.iter();
                let first = it.next().unwrap();
                let second = it.next().unwrap();
                Some((first, second))
            }
            _ => None,
        }
    });
    tp3.and_then(|generic_arg| match generic_arg {
        (&GenericArgument::Type(ref ty1), &GenericArgument::Type(ref ty2)) => Some((ty1, ty2)),
        _ => None,
    })
}

pub fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    extract_inner_from_bracketed_type(
        ty,
        vec![
            "Option|".to_string(),
            "std|option|Option|".into(),
            "core|option|Option|".into(),
        ],
    )
}

pub fn extract_type_from_vector(ty: &syn::Type) -> Option<&syn::Type> {
    extract_inner_from_bracketed_type(
        ty,
        vec![
            "Vector|".to_string(),
            "std|vec|Vec|".into(),
            "Vec|".to_string(),
        ],
    )
}

pub fn extract_type_from_map(ty: &syn::Type) -> Option<(&syn::Type, &syn::Type)> {
    extract_inner_from_double_bracketed_type(
        ty,
        vec!["BTreeMap|".to_string(), "std|collections|BTreeMap|".into()],
    )
}

pub fn extract_type_from_linked_hash_map(ty: &syn::Type) -> Option<(&syn::Type, &syn::Type)> {
    extract_inner_from_double_bracketed_type(
        ty,
        vec![
            "LinkedHashMap|".to_string(),
            "linked_hash_map|LinkedHashMap|".into(),
        ],
    )
}
pub fn extract_type_from_aorist_ref(ty: &syn::Type) -> Option<&syn::Type> {
    extract_inner_from_bracketed_type(ty, vec!["Arc|".to_string(), "AoristRef|".to_string()])
}

use aorist_util::{get_raw_objects_of_type, read_file};
use codegen::Scope;
use serde_yaml::{Value};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

fn process_attributes(raw_objects: &Vec<HashMap<String, Value>>) {
    let attributes = get_raw_objects_of_type(raw_objects, "Attribute".into());
    let mut scope = Scope::new();
    scope.import("aorist_primitives", "define_attribute");
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");
    //scope.import("std::sync", "Arc");
    //scope.import("std::sync", "RwLock");
    //scope.import("crate::concept", "AoristConcept");
    //scope.import("crate::concept", "WrappedConcept");
    //scope.import("crate::concept", "Concept");
    //scope.import("crate::constraint", "Constraint");
    //scope.import("aorist_concept", "Constrainable");
    //scope.import("aorist_concept", "ConstrainableWithChildren");
    //scope.import("aorist_concept", "aorist_concept");
    //scope.import("aorist_concept", "InnerObject");
    //scope.import("uuid", "Uuid");
    //scope.import("derivative", "Derivative");
    scope.import("pyo3::prelude", "*");
    //scope.import("paste", "paste");

    let sql_derive_macros = attributes
        .iter()
        .map(|x| x.get("sql").unwrap().as_str().unwrap().to_string())
        .collect::<HashSet<_>>();
    let orc_derive_macros = attributes
        .iter()
        .map(|x| x.get("orc").unwrap().as_str().unwrap().to_string())
        .collect::<HashSet<_>>();
    let presto_derive_macros = attributes
        .iter()
        .map(|x| x.get("presto").unwrap().as_str().unwrap().to_string())
        .collect::<HashSet<_>>();
    let sqlite_derive_macros = attributes
        .iter()
        .map(|x| x.get("sqlite").unwrap().as_str().unwrap().to_string())
        .collect::<HashSet<_>>();
    let postgres_derive_macros = attributes
        .iter()
        .map(|x| x.get("postgres").unwrap().as_str().unwrap().to_string())
        .collect::<HashSet<_>>();
    let bigquery_derive_macros = attributes
        .iter()
        .map(|x| x.get("bigquery").unwrap().as_str().unwrap().to_string())
        .collect::<HashSet<_>>();

    let derive_macros = sql_derive_macros
        .into_iter()
        .chain(orc_derive_macros.into_iter())
        .chain(presto_derive_macros.into_iter())
        .chain(sqlite_derive_macros.into_iter())
        .chain(postgres_derive_macros.into_iter())
        .chain(bigquery_derive_macros.into_iter())
        .collect::<HashSet<_>>();

    for item in derive_macros {
        scope.import("aorist_derive", &item);
    }
    let mut attribute_names: Vec<String> = Vec::new();
    for attribute in attributes {
        let name = attribute.get("name").unwrap().as_str().unwrap().to_string();
        let orc = attribute.get("orc").unwrap().as_str().unwrap().to_string();
        let value = attribute
            .get("value")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let presto = attribute
            .get("presto")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let sql = attribute.get("sql").unwrap().as_str().unwrap().to_string();
        let sqlite = attribute
            .get("sqlite")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let postgres = attribute
            .get("postgres")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let bigquery = attribute
            .get("bigquery")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let define = format!(
            "define_attribute!({}, {}, {}, {}, {}, {}, {}, {});",
            name, orc, presto, sql, sqlite, postgres, bigquery, value
        );
        scope.raw(&define);
        attribute_names.push(name.clone());
    }
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("attributes.rs");

    fs::write(&dest_path, scope.to_string()).unwrap();
}

fn main() {
    let _file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("constrainables.txt")
        .unwrap();

    let raw_objects = read_file("attributes.yaml");
    process_attributes(&raw_objects);
}

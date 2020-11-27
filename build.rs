use codegen::Scope;
use serde_yaml::{from_str, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

pub fn read_file(filename: &str) -> Vec<HashMap<String, Value>> {
    let s = fs::read_to_string(filename).unwrap();
    s.split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| from_str(x).unwrap())
        .collect()
}

fn main() {
    let raw_objects = read_file("basic.yaml");
    let attributes = raw_objects
        .into_iter()
        .filter(|x| x.get("type").unwrap().as_str().unwrap() == "Attribute")
        .map(|x| {
            x.get("spec")
                .unwrap()
                .as_mapping()
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k.as_str().unwrap().into(), v.clone()))
                .collect()
        })
        .collect::<Vec<HashMap<String, Value>>>();

    assert_eq!(attributes.len(), 1);

    let mut scope = Scope::new();
    scope.import("aorist_primitives", "define_attribute");
    scope.import("aorist_primitives", "register_attribute");
    scope.import("aorist_primitives", "TAttribute");
    scope.import("aorist_primitives", "TOrcAttribute");
    scope.import("aorist_primitives", "TPrestoAttribute");
    scope.import("aorist_primitives", "TSQLAttribute");
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");
    scope.import("sqlparser::ast", "DataType");

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

    let derive_macros = sql_derive_macros
        .into_iter()
        .chain(orc_derive_macros.into_iter())
        .chain(presto_derive_macros.into_iter())
        .collect::<HashSet<_>>();

    for item in derive_macros {
        scope.import("aorist_derive", &item);
    }
    let mut attribute_names: Vec<String> = Vec::new();
    for attribute in attributes {
        let name = attribute.get("name").unwrap().as_str().unwrap().to_string();
        let orc = attribute.get("orc").unwrap().as_str().unwrap().to_string();
        let presto = attribute
            .get("presto")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let sql = attribute.get("sql").unwrap().as_str().unwrap().to_string();

        let define = format!("define_attribute!({}, {}, {}, {});", name, orc, presto, sql);
        scope.raw(&define);
        attribute_names.push(name.clone());
    }
    let register = format!("register_attribute!(Attribute1, {});", attribute_names.join(", "));
    scope.raw(&register);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");

    fs::write(&dest_path, scope.to_string()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

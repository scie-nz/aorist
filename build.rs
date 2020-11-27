use codegen::Scope;
use serde_yaml::{from_str, Value};
use std::collections::HashMap;
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

    let mut scope = Scope::new();
    for attribute in attributes {
        let attribute_name = attribute.get("name").unwrap().as_str().unwrap().into();
        scope.new_struct(attribute_name).derive("Debug");
    }
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");

    fs::write(&dest_path, scope.to_string()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

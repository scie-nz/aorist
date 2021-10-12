use aorist_util::{get_raw_objects_of_type, read_file};
use codegen::Scope;
use serde_yaml::Value;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

fn process_attributes_py(raw_objects: &Vec<HashMap<String, Value>>) {
    let attributes = get_raw_objects_of_type(raw_objects, "Attribute".into());
    let mut scope_py = Scope::new();
    let fun = scope_py
        .new_fn("attributes_module")
        .vis("pub")
        .ret("PyResult<()>")
        .arg("_py", "Python")
        .arg("m", "&PyModule");
    for attribute in attributes {
        let name = attribute.get("name").unwrap().as_str().unwrap().to_string();
        let export = format!("m.add_class::<{}>().unwrap();", name,);
        fun.line(&export);
    }
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path_py = Path::new(&out_dir).join("python.rs");
    fun.line("Ok(())");
    fs::write(&dest_path_py, scope_py.to_string()).unwrap();
}
fn process_attributes(raw_objects: &Vec<HashMap<String, Value>>) {
    let attributes = get_raw_objects_of_type(raw_objects, "Attribute".into());
    let mut scope = Scope::new();
    #[cfg(feature = "python")]
    scope.import("aorist_primitives", "define_attribute");
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");

    let sql_derive_macros;
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

    cfg_if::cfg_if! {
        if #[cfg(feature = "sql")] {
            sql_derive_macros = attributes
                .iter()
                .map(|x| x.get("sql").unwrap().as_str().unwrap().to_string())
                .collect::<HashSet<_>>();
        } else {
            sql_derive_macros = HashSet::new();
        }
    }
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
        let key = format!(
            "{}",
            match attribute.get("key") {
                Some(x) => x.as_bool().unwrap(),
                None => false,
            }
        )
        .to_string();
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
        let python = match attribute
            .get("python")
            .unwrap()
            .as_str().unwrap() {
                "str" => "pyo3::types::PyString",
                "int" => "pyo3::types::PyLong",
                "float" => "pyo3::types::PyFloat",
                _ => panic!("Only str, int or float python types supported."),
            }.to_string();

        let define = format!(
            "define_attribute!({}, {}, {}, {}, {}, {}, {}, {}, {}, {});",
            name, orc, presto, sql, sqlite, postgres, bigquery, value, key, python
        );
        scope.raw(&define);
        attribute_names.push(name.clone());
    }
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("attributes.rs");
    fs::write(&dest_path, scope.to_string()).unwrap();
}

fn main() {
    println!("cargo:rustc-cfg=feature=\"build-time\"");
    let raw_objects = read_file("attributes.yaml");
    process_attributes(&raw_objects);
    #[cfg(feature = "python")]
    process_attributes_py(&raw_objects);
}

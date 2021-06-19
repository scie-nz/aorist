use codegen::Scope;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    attributes: Vec<String>,
}

fn process_attributes(attribute_names: Vec<String>) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("attributes.rs");
    let mut scope = Scope::new();
    scope.import("aorist_primitives", "register_attribute_new");
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");
    for attribute in &attribute_names {
        scope.import("aorist_attributes", attribute);
    }
    let register = format!(
        "register_attribute_new!(Attribute, {});",
        attribute_names.join(", ")
    );
    scope.raw(&register);

    fs::write(&dest_path, scope.to_string()).unwrap();
}

fn main() {
    let toml_str = fs::read_to_string("aorist.toml").unwrap();
    let decoded: Config = toml::from_str(&toml_str).unwrap();
    process_attributes(decoded.attributes);
}

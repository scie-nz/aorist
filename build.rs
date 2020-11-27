use codegen::Scope;
use std::env;
use std::fs;
use std::path::Path;
use serde_yaml::{from_str, Value};
use std::collections::HashMap;

pub fn read_file(filename: &str) -> Vec<HashMap<String, Value>> {
    let s = fs::read_to_string(filename).unwrap();
    s
        .split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| from_str(x).unwrap())
        .collect()
}

fn main() {
	let _raw_objects = read_file("basic.yaml");

  let out_dir = env::var_os("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("hello.rs");    
	
	let mut scope = Scope::new();
    scope.new_struct("Foo")
        .derive("Debug")
        .field("one", "usize");
    fs::write(&dest_path, scope.to_string()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

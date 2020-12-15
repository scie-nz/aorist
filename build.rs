use codegen::Scope;
use indoc::formatdoc;
use serde_yaml::{from_str, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

pub fn read_file(filename: &str) -> Vec<HashMap<String, Value>> {
    let s = fs::read_to_string(filename).unwrap();
    s.split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| from_str(x).unwrap())
        .collect()
}

fn get_raw_objects_of_type(
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

fn process_constraints(raw_objects: &Vec<HashMap<String, Value>>) {
    let constraints = get_raw_objects_of_type(raw_objects, "Constraint".into());
    let mut scope = Scope::new();
    for constraint in &constraints {
        scope.import("crate", constraint.get("root").unwrap().as_str().unwrap());
    }
    let mut dependencies: HashMap<(String, String), Vec<String>> = HashMap::new();
    for constraint in constraints {
        let name = constraint
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let root = constraint
            .get("root")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let required = match constraint.get("requires") {
            None => Vec::new(),
            Some(required) => required
                .clone()
                .as_sequence()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        };
        dependencies.insert((name, root), required);
    }
    let constraint_names: HashSet<String> = dependencies.keys().map(|x| x.0.clone()).collect();
    for dep in dependencies.values() {
        for elem in dep {
            if !constraint_names.contains(elem) {
                panic!("Cannot find definition for required constraint {}", elem);
            }
        }
    }

    let mut g: HashMap<(String, String), HashSet<String>> = dependencies
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.clone().into_iter().collect::<HashSet<String>>(),
            )
        })
        .collect();

    let mut leaf_name = g
        .iter()
        .filter(|(_, v)| v.len() == 0)
        .map(|(k, _)| k)
        .next();
    let mut order: Vec<(String, String)> = Vec::new();
    while let Some(val) = leaf_name {
        let key = val.clone();
        println!("key: {}, {}", key.0, key.1);
        g.remove(&key);
        for (k, x) in g.iter_mut() {
            println!(
                "node: {}, dependencies: {}",
                k.0,
                x.clone().into_iter().collect::<Vec<String>>().join(", ")
            );
            x.remove(&key.0);
        }
        order.push(key);
        leaf_name = g
            .iter()
            .filter(|(_, v)| v.len() == 0)
            .map(|(k, _)| k)
            .next();
        if g.len() == 0 {
            break;
        }
    }
    if g.len() > 0 {
        panic!("Cycles in constraint dependencies are not allowed!");
    }
    for (name, root) in &order {
        let required = dependencies.get(&(name.clone(), root.clone())).unwrap();
        let define = match required.len() {
            0 => format!("define_constraint!({}, {});", name, root),
            _ => format!(
                "define_constraint!({}, {}, {});",
                name,
                root,
                required.join(", ")
            ),
        };
        scope.raw(&define);
    }
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constraints.rs");
    scope.raw(&format!(
        "register_constraint!(AoristConstraint, {});",
        order
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<_>>()
            .join("\n,    ")
    ));
    fs::write(&dest_path, scope.to_string()).unwrap();
}

fn process_attributes(raw_objects: &Vec<HashMap<String, Value>>) {
    let attributes = get_raw_objects_of_type(raw_objects, "Attribute".into());
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
    scope.import("std::rc", "Rc");
    scope.import("crate::concept", "AoristConcept");
    scope.import("crate::constraint", "Constraint");
    scope.import("aorist_concept", "Constrainable");

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
    let register = format!(
        "register_attribute!(Attribute, {});",
        attribute_names.join(", ")
    );
    scope.raw(&register);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("attributes.rs");

    fs::write(&dest_path, scope.to_string()).unwrap();
}

fn get_match_arms(dialects: HashMap<String, HashMap<String, Value>>) -> String {
    dialects
        .into_iter()
        .map(|(dialect, config)| {
            let params: Vec<(String, String)> = config
                .get("parameters")
                .unwrap()
                .as_mapping()
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k.as_str().unwrap().into(), v.as_str().unwrap().into()))
                .collect();

            format!(
                "Dialect::{dialect}{{..}} => Ok(format!(\"{call}({param_names})\", {params}).to_string()),",
                dialect = dialect,
                call = config.get("call").unwrap().as_str().unwrap().replace("{","{{").replace("}","}}"),
                param_names = params
                    .iter()
                    .map(|(k, _)| format!("{{{}}}", k).to_string())
                    .collect::<Vec<_>>()
                    .join(", ".into()),
                params = params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v).to_string())
                    .collect::<Vec<_>>()
                    .join(", ".into()),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let _file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("constrainables.txt")
        .unwrap();

    let raw_objects = read_file("basic.yaml");
    process_attributes(&raw_objects);
    process_constraints(&raw_objects);

    let programs = get_raw_objects_of_type(&raw_objects, "Program".into());
    assert_eq!(programs.len(), 1);
    let mut scope = Scope::new();
    let mut by_uses: HashMap<String, HashMap<String, HashMap<String, HashMap<String, Value>>>> =
        HashMap::new();
    let mut program_uses = HashSet::new();
    let mut roots = HashSet::new();
    for x in programs.into_iter() {
        let program_use = x.get("use").unwrap().as_str().unwrap().to_string();
        let root = x.get("root").unwrap().as_str().unwrap().to_string();
        let root_crate = x.get("crate").unwrap().as_str().unwrap().to_string();
        program_uses.insert(program_use.clone());
        roots.insert((root_crate, root.clone()));
        let dialect = x.get("dialect").unwrap().as_str().unwrap().to_string();
        by_uses
            .entry(root)
            .or_insert(HashMap::new())
            .entry(program_use)
            .or_insert(HashMap::new())
            .entry(dialect)
            .or_insert(x);
    }
    scope.import("aorist_primitives", "Dialect");
    for program_use in program_uses {
        scope.import("aorist_primitives", &program_use);
    }
    for (root_crate, root) in roots {
        scope.import(&format!("crate::{}", &root_crate), &root);
    }
    scope.raw(
        &by_uses
            .into_iter()
            .map(|(root, program_uses)| {
                program_uses
                    .into_iter()
                    .map(|(program_use, dialects)| {
                        let match_arms = get_match_arms(dialects);

                        formatdoc!(
                            "
                impl {program_use} for {struct_name} {{
                    fn get_call(&self, dialect: Dialect) -> Result<String, String> {{
                        match dialect {{
                            {match_arms}
                            _ => Err(format!(\"Dialect {{:?}} not supported\", dialect).into())
                        }}
                    }}
                }}
            ",
                            program_use = program_use,
                            struct_name = root,
                            match_arms = match_arms,
                        )
                        .to_string()
                    })
                    .collect::<Vec<_>>()
                    .join("\n".into())
            })
            .collect::<Vec<_>>()
            .join("\n".into()),
    );
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("programs.rs");

    fs::write(&dest_path, scope.to_string()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=basic.yaml");
    println!("cargo:rerun-if-changed=constrainables.txt");
}

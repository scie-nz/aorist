use aorist_util::{get_raw_objects_of_type, read_file};
use codegen::Scope;
use serde_yaml::Value;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

type ConstraintTuple = (String, String, Option<String>, Option<String>);

fn process_constraints_py(raw_objects: &Vec<HashMap<String, Value>>) {
    let constraints = get_raw_objects_of_type(raw_objects, "Constraint".into());
    let mut scope_py = Scope::new();
    let fun = scope_py
        .new_fn("constraints_module")
        .vis("pub")
        .ret("PyResult<()>")
        .arg("_py", "Python")
        .arg("m", "&PyModule");
    for attribute in constraints {
        let name = attribute.get("name").unwrap().as_str().unwrap().to_string();
        let export = format!("m.add_class::<{}>().unwrap();", name,);
        fun.line(&export);
    }
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path_py = Path::new(&out_dir).join("python.rs");
    fun.line("Ok(())");
    fs::write(&dest_path_py, scope_py.to_string()).unwrap();
}
fn get_constraint_dependencies(
    constraints: &Vec<HashMap<String, Value>>,
) -> HashMap<ConstraintTuple, Vec<String>> {
    let mut dependencies: HashMap<ConstraintTuple, Vec<String>> = HashMap::new();
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
        let title = match constraint.get("title") {
            Some(x) => Some(x.as_str().unwrap().to_string()),
            None => None,
        };
        let body = match constraint.get("body") {
            Some(x) => Some(x.as_str().unwrap().to_string()),
            None => None,
        };
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
        dependencies.insert((name, root, title, body), required);
    }
    let constraint_names: HashSet<String> = dependencies.keys().map(|x| x.0.clone()).collect();
    for dep in dependencies.values() {
        for elem in dep {
            if !constraint_names.contains(elem) {
                panic!("Cannot find definition for required constraint {}", elem);
            }
        }
    }
    dependencies
}

fn compute_topological_sort(
    dependencies: &HashMap<ConstraintTuple, Vec<String>>,
) -> Vec<ConstraintTuple> {
    let mut g: HashMap<ConstraintTuple, HashSet<String>> = dependencies
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
    let mut order: Vec<_> = Vec::new();
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
    order
}

fn process_constraints(raw_objects: &Vec<HashMap<String, Value>>) {
    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("constraints.txt")
        .unwrap();
    let constraints = get_raw_objects_of_type(raw_objects, "Constraint".into());
    let mut scope = Scope::new();
    scope.import("uuid", "Uuid");
    for constraint in &constraints {
        scope.import(
            "aorist_core",
            constraint.get("root").unwrap().as_str().unwrap(),
        );
    }
    let dependencies = get_constraint_dependencies(&constraints);
    let order = compute_topological_sort(&dependencies);
    let program_required = constraints
        .iter()
        .map(|x| {
            (
                x.get("name").unwrap().as_str().unwrap().to_string(),
                match x.get("requiresProgram") {
                    Some(Value::Bool(ref val)) => *val,
                    None => false,
                    _ => panic!("requiresProgram needs to be a bool"),
                },
            )
        })
        .collect::<HashMap<String, bool>>();
    let attach_if = constraints
        .iter()
        .map(|x| {
            (
                x.get("name").unwrap().as_str().unwrap().to_string(),
                match x.get("attachIf") {
                    Some(Value::String(ref val)) => Some(val.to_string()),
                    None => None,
                    _ => panic!("attachIf needs to be a string containing a Rust closure"),
                },
            )
        })
        .collect::<HashMap<String, Option<String>>>();
    let constraint_closures = constraints
        .iter()
        .map(|x| {
            (
                x.get("name").unwrap().as_str().unwrap().to_string(),
                match x.get("requiredConstraintsClosure") {
                    Some(Value::String(ref val)) => Some(val.to_string()),
                    None => None,
                    _ => panic!(
                        "requiredConstraintsClosure needs to be a string containing a Rust closure"
                    ),
                },
            )
        })
        .collect::<HashMap<String, Option<String>>>();
    let mut satisfiable = Vec::new();
    for (name, root, title, body) in &order {
        let required = dependencies
            .get(&(name.clone(), root.clone(), title.clone(), body.clone()))
            .unwrap();
        let requires_program = program_required.get(name).unwrap();
        if *requires_program {
            satisfiable.push(name.clone());
        }
        let should_add = attach_if.get(name).unwrap();
        let get_required = constraint_closures.get(name).unwrap();
        let formatted_title = match title {
            Some(x) => format!("Some(\"{}\".to_string())", x),
            None => "None".to_string(),
        };
        let formatted_body = match body {
            Some(x) => format!("Some(\"{}\".to_string())", x),
            None => "None".to_string(),
        };
        let should_add = match should_add {
            None => "|_, _| true".to_string(),
            Some(x) => x.to_string(),
        };
        let get_required = match get_required {
            None => "|_, _| Vec::new()".to_string(),
            Some(x) => x.to_string(),
        };

        let define = match required.len() {
            0 => format!(
                "define_constraint!({}, {}, Satisfy{}, {}, Constraint, {}, {}, {}, {});",
                name,
                requires_program,
                name,
                root,
                formatted_title,
                formatted_body,
                should_add,
                get_required
            ),
            _ => format!(
                "define_constraint!({}, {}, Satisfy{}, {}, Constraint, {}, {}, {}, {}, {});",
                name,
                requires_program,
                name,
                root,
                formatted_title,
                formatted_body,
                should_add,
                get_required,
                required.join(", ")
            ),
        };
        scope.raw(&define);
    }
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constraints.rs");
    scope.raw(&format!(
        "register_constraint_new!(AoristConstraint, 'a, {});",
        order
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<_>>()
            .join("\n,    ")
    ));
    fs::write(&dest_path, scope.to_string()).unwrap();
    for (name, _, _, _) in &order {
        writeln!(
            file,
            "node [shape = box, color=red, fontname = Helvetica, fontcolor=red] '{}';",
            name
        )
        .unwrap();
    }
    for (name, root, title, body) in &order {
        writeln!(file, "'{}'->'{}'[color=red];", name, root,).unwrap();
        let required = dependencies
            .get(&(name.clone(), root.clone(), title.clone(), body.clone()))
            .unwrap();
        for req in required {
            writeln!(file, "'{}'->'{}'[color=red];", name, req).unwrap();
        }
    }
}

fn main() {
    let raw_objects = read_file("constraints.yaml");
    process_constraints(&raw_objects);
    process_constraints_py(&raw_objects);
}

use aorist_error::{AResult, AoristError};
use crate::get_raw_objects_of_type;
use codegen::Scope;
use serde_yaml::Value;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;
pub type ConstraintTuple = (String, String, Option<String>, Option<String>);

pub fn get_env_var(name: &str) -> AResult<std::ffi::OsString> {
    env::var_os(name).ok_or_else(|| {
        AoristError::UnexpectedNoneError(format!("Could not find envrionment variable {}", name))
    })
}

fn get_constraint_field(constraint: &HashMap<String, Value>, field: &str) -> AResult<String> {
    Ok(constraint
        .get(field)
        .ok_or_else(|| {
            AoristError::UnexpectedNoneError(format!(
                "Could not find '{}' field in constraint definition.",
                field
            ))
        })?
        .as_str()
        .ok_or_else(|| {
            AoristError::CannotConvertJSONError(
                "Cannot convert constraint name field to string.".into(),
            )
        })?
        .to_string())
}

pub fn process_constraints_py(raw_objects: &Vec<HashMap<String, Value>>) -> AResult<()> {
    let constraints = get_raw_objects_of_type(raw_objects, "Constraint".into())?;
    let parsed_constraints = constraints
        .clone()
        .into_iter()
        .map(|x| ParsedConstraintDef::new(x))
        .collect::<AResult<Vec<ParsedConstraintDef>>>()?;
    let mut scope_py = Scope::new();
    let fun = scope_py
        .new_fn("constraints_module")
        .vis("pub")
        .ret("PyResult<()>")
        .arg("_py", "Python")
        .arg("m", "&PyModule");
    for constraint in parsed_constraints {
        let export = format!("m.add_class::<{}>()?;", &constraint.name,);
        fun.line(&export);
    }
    fun.line("m.add_class::<crate::constraint::AoristConstraintProgram>()?;");
    let out_dir = get_env_var("OUT_DIR")?;
    let dest_path_py = Path::new(&out_dir).join("python.rs");
    fun.line("Ok(())");
    fs::write(&dest_path_py, scope_py.to_string())?;
    Ok(())
}
pub fn get_constraint_dependencies(
    constraints: &HashMap<ConstraintTuple, ParsedConstraintDef>,
) -> AResult<HashMap<ConstraintTuple, Vec<String>>> {
    let mut dependencies: HashMap<ConstraintTuple, Vec<String>> = HashMap::new();
    for (key, constraint) in constraints.iter() {
        dependencies.insert(key.clone(), constraint.get_required());
    }
    let constraint_names: HashSet<String> = dependencies.keys().map(|x| x.0.clone()).collect();
    for dep in dependencies.values() {
        for elem in dep.iter() {
            if !constraint_names.contains(elem) {
                panic!("Cannot find definition for required constraint {}", elem);
            }
        }
    }
    Ok(dependencies)
}

pub fn compute_topological_sort(
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

pub struct ParsedConstraintDef {
    pub name: String,
    pub root: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub required: Vec<String>,
    pub requires_program: bool,
    pub attach_if: Option<String>,
    pub required_constraints_closure: Option<String>,
}
impl ParsedConstraintDef {
    pub fn get_formatted_body(&self) -> String {
        match &self.body {
            Some(x) => format!("AOption(ROption::RSome(\"{}\".into()))", x),
            None => "AOption(ROption::RNone)".to_string(),
        }
    }
    pub fn get_formatted_title(&self) -> String {
        match &self.title {
            Some(x) => format!("AOption(ROption::RSome(\"{}\".into()))", x),
            None => "AOption(ROption::RNone)".to_string(),
        }
    }
    pub fn get_formatted_attach_if(&self) -> String {
        match &self.attach_if {
            None => "|_, _| true".to_string(),
            Some(x) => x.to_string(),
        }
    }
    pub fn get_required(&self) -> Vec<String> {
        self.required.clone()
    }
    pub fn get_formatted_required_constraints_closure(&self) -> String {
        match &self.required_constraints_closure {
            None => "|_, _| Vec::new()".to_string(),
            Some(x) => x.to_string(),
        }
    }
    pub fn get_define_constraint_abi(&self) -> String {
        match self.required.len() {
            0 => format!("define_constraint_abi!({});", self.name,),
            _ => format!(
                "define_constraint_abi!({}, {});",
                self.name,
                self.required.join(", ")
            ),
        }
    }
    pub fn get_define_constraint(&self) -> String {
        let required = self.get_required();
        let formatted_title = self.get_formatted_title();
        let formatted_body = self.get_formatted_body();
        let attach_if = self.get_formatted_attach_if();
        let get_required = self.get_formatted_required_constraints_closure();

        match required.len() {
            0 => format!(
                "define_constraint!({}, {}, Satisfy{}, {}, Constraint, {}, {}, {}, {});",
                self.name,
                self.requires_program,
                self.name,
                self.root,
                formatted_title,
                formatted_body,
                attach_if,
                get_required
            ),
            _ => format!(
                "define_constraint!({}, {}, Satisfy{}, {}, Constraint, {}, {}, {}, {}, {});",
                self.name,
                self.requires_program,
                self.name,
                self.root,
                formatted_title,
                formatted_body,
                attach_if,
                get_required,
                required.join(", ")
            ),
        }
    }
    pub fn get_key(&self) -> ConstraintTuple {
        (
            self.name.clone(),
            self.root.clone(),
            self.title.clone(),
            self.body.clone(),
        )
    }
    fn new(constraint: HashMap<String, Value>) -> AResult<Self> {
        let name = get_constraint_field(&constraint, "name")?;
        let root = get_constraint_field(&constraint, "root")?;
        let title = match get_constraint_field(&constraint, "title") {
            Ok(x) => Some(x),
            Err(_) => None,
        };
        let body = match get_constraint_field(&constraint, "body") {
            Ok(x) => Some(x),
            Err(_) => None,
        };
        let attach_if = match get_constraint_field(&constraint, "attachIf") {
            Ok(x) => Some(x),
            Err(_) => None,
        };
        let required_constraints_closure =
            match get_constraint_field(&constraint, "requiredConstraintsClosure") {
                Ok(x) => Some(x),
                Err(_) => None,
            };
        let required = match constraint.get("requires") {
            None => Vec::new(),
            Some(required) => required
                .clone()
                .as_sequence()
                .ok_or_else(|| {
                    AoristError::CannotConvertJSONError(format!(
                        "Cannot convert requires vector in constraint {}",
                        name
                    ))
                })?
                .iter()
                .map(|x| {
                    x.as_str().ok_or_else(|| {
                        AoristError::CannotConvertJSONError(format!(
                            "Cannot convert field {:?} to string.",
                            x
                        ))
                    })
                })
                .collect::<AResult<Vec<&str>>>()?
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        };
        let requires_program = match constraint.get("requiresProgram") {
            Some(Value::Bool(ref val)) => Ok(*val),
            None => Ok(false),
            _ => Err(AoristError::CannotConvertJSONError(
                "requiresProgram needs to be a bool".into(),
            )),
        }?;
        Ok(Self {
            name,
            root,
            title,
            body,
            required,
            requires_program,
            required_constraints_closure,
            attach_if,
        })
    }
}

pub fn parse_and_sort_constraints(
    raw_objects: &Vec<HashMap<String, Value>>,
) -> AResult<Vec<ParsedConstraintDef>> {
    let constraints = get_raw_objects_of_type(raw_objects, "Constraint".into())?;
    let parsed_constraints = constraints
        .clone()
        .into_iter()
        .map(|x| ParsedConstraintDef::new(x))
        .collect::<AResult<Vec<ParsedConstraintDef>>>()?;
    let mut constraints_map = parsed_constraints
        .into_iter()
        .map(|x| (x.get_key(), x))
        .collect::<HashMap<ConstraintTuple, ParsedConstraintDef>>();
    let dependencies = get_constraint_dependencies(&constraints_map)?;
    let order = compute_topological_sort(&dependencies);
    let mut out = Vec::new();
    for key in order.into_iter() {
        let constraint = constraints_map.remove(&key).ok_or_else(|| {
            AoristError::UnexpectedNoneError(format!(
                "Constraint with key {:?} not found in map.",
                key
            ))
        })?;
        out.push(constraint);
    }
    Ok(out)
}

pub fn process_constraints(raw_objects: &Vec<HashMap<String, Value>>) -> AResult<()> {
    let mut scope = Scope::new();
    scope.import("aorist_util", "AUuid");
    let parsed = parse_and_sort_constraints(raw_objects)?;

    for constraint in parsed.iter() {
        scope.import("scienz", &constraint.root);
    }
    for constraint in parsed.iter() {
        let define = constraint.get_define_constraint();
        scope.raw(&define);
    }
    let out_dir = get_env_var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("constraints.rs");
    scope.raw(&format!(
        "register_constraint_new!(AoristConstraint, 'a, {});",
        parsed
            .iter()
            .map(|x| x.name.clone())
            .collect::<Vec<String>>()
            .join("\n,    ")
    ));
    fs::write(&dest_path, scope.to_string())?;
    Ok(())
}

pub fn process_constraints_new(raw_objects: &Vec<HashMap<String, Value>>) -> AResult<()> {
    let mut scope = Scope::new();
    scope.import("aorist_primitives", "register_constraint");
    scope.import("aorist_primitives", "define_constraint_abi");
    let parsed = parse_and_sort_constraints(raw_objects)?;
    for constraint in parsed.iter() {
        scope.import("scienz", &constraint.root);
    }
    for constraint in parsed.iter() {
        let define = constraint.get_define_constraint_abi();
        scope.raw(&define);
    }
    let out_dir = get_env_var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("constraints.rs");
    scope.raw(&format!(
        "register_constraint!(AoristConstraint, 'a, {});",
        parsed
            .iter()
            .map(|x| x.name.clone())
            .collect::<Vec<String>>()
            .join("\n,    ")
    ));
    fs::write(&dest_path, scope.to_string())?;
    Ok(())
}

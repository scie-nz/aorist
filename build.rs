extern crate bindgen;

use aorist_util::{get_raw_objects_of_type, read_file};
use codegen::Scope;
use indoc::formatdoc;
use inflector::cases::snakecase::to_snake_case;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_str, Value};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AncestorArgument {
    call: String,
    attaches: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MultipleAncestorsArgument {
    call: String,
    attaches: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Program {
    r#use: String,
    root: String,
    dialect: String,
    preamble: Option<String>,
    call: String,
    args: Option<Vec<Arg>>,
    kwargs: Option<HashMap<String, Arg>>,
    pip_requires: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
enum BuildObject {
    Program(Program),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
enum Arg {
    AncestorArgument(AncestorArgument),
    MultipleAncestorsArgument(MultipleAncestorsArgument),
}

type ConstraintTuple = (String, String, Option<String>, Option<String>);

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
        scope.import("crate", constraint.get("root").unwrap().as_str().unwrap());
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
    for (name, root, title, body) in &order {
        let required = dependencies
            .get(&(name.clone(), root.clone(), title.clone(), body.clone()))
            .unwrap();
        let requires_program = program_required.get(name).unwrap();
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
        "register_constraint!(AoristConstraint, 'a, 'b, {});",
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

fn process_attributes(raw_objects: &Vec<HashMap<String, Value>>) {
    let attributes = get_raw_objects_of_type(raw_objects, "Attribute".into());
    let mut scope = Scope::new();
    scope.import("aorist_primitives", "define_attribute");
    scope.import("aorist_primitives", "register_attribute");
    scope.import("serde", "Serialize");
    scope.import("serde", "Deserialize");
    scope.import("std::sync", "Arc");
    scope.import("std::sync", "RwLock");
    scope.import("crate::concept", "AoristConcept");
    scope.import("crate::concept", "Concept");
    scope.import("crate::constraint", "Constraint");
    scope.import("aorist_concept", "Constrainable");
    scope.import("aorist_concept", "ConstrainableWithChildren");
    scope.import("aorist_concept", "aorist_concept");
    scope.import("aorist_concept", "InnerObject");
    scope.import("uuid", "Uuid");
    scope.import("derivative", "Derivative");
    scope.import("pyo3::prelude", "*");
    scope.import("paste", "paste");

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
    let register = format!(
        "register_attribute!(Attribute, {});",
        attribute_names.join(", ")
    );
    scope.raw(&register);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("attributes.rs");

    fs::write(&dest_path, scope.to_string()).unwrap();
}

fn process_concepts() {
    let mut scope = Scope::new();

    let concepts = vec![
        ("crate::access_policy", "AccessPolicy"),
        ("crate::access_policy", "ApproveAccessSelector"),
        ("crate::algorithms", "RegressionAlgorithm"),
        ("crate::algorithms", "RandomForestRegressionAlgorithm"),
        ("crate::algorithms", "SVMRegressionAlgorithm"),
        ("crate::asset", "Asset"),
        ("crate::asset", "StaticDataTable"),
        ("crate::asset", "SupervisedModel"),
        ("crate::asset", "DerivedAsset"),
        ("crate::attributes", "Attribute"),
        ("crate::attributes", "Predicate"),
        ("crate::layout", "APILayout"),
        ("crate::layout", "APIOrFileLayout"),
        ("crate::layout", "FileBasedStorageLayout"),
        ("crate::layout", "SingleFileLayout"),
        ("crate::layout", "PushshiftSubredditPostsAPILayout"),
        ("crate::layout", "TabularLayout"),
        ("crate::layout", "DynamicTabularLayout"),
        ("crate::layout", "StaticTabularLayout"),
        ("crate::layout", "Granularity"),
        ("crate::layout", "DailyGranularity"),
        ("crate::dataset", "DataSet"),
        ("crate::role", "Role"),
        ("crate::role", "GlobalPermissionsAdmin"),
        ("crate::compression", "GzipCompression"),
        ("crate::features::objective", "Foo"),
        ("crate::features::objective", "Bar"),
        ("crate::compression", "DataCompression"),
        ("crate::compression", "ZipCompression"),
        ("crate::compliance", "ComplianceConfig"),
        ("crate::features", "ContinuousRegressionObjective"),
        ("crate::features", "ContinuousObjective"),
        ("crate::features", "RegressionObjective"),
        ("crate::header", "CSVHeader"),
        ("crate::header", "FileHeader"),
        ("crate::location", "AlluxioLocation"),
        ("crate::location", "BigQueryLocation"),
        ("crate::location", "GCSLocation"),
        ("crate::location", "GithubLocation"),
        ("crate::location", "HiveLocation"),
        ("crate::location", "LocalFileSystemLocation"),
        ("crate::location", "OnPremiseLocation"),
        ("crate::location", "MinioLocation"),
        ("crate::location", "S3Location"),
        ("crate::location", "PostgresLocation"),
        ("crate::location", "PushshiftAPILocation"),
        ("crate::location", "RemoteLocation"),
        ("crate::location", "SQLiteLocation"),
        ("crate::location", "WebLocation"),
        ("crate::models", "Model"),
        ("crate::models", "SingleObjectiveRegressor"),
        ("crate::encoding", "GDBEncoding"),
        ("crate::encoding", "CSVEncoding"),
        ("crate::encoding", "TSVEncoding"),
        ("crate::encoding", "Encoding"),
        ("crate::encoding", "JSONEncoding"),
        ("crate::encoding", "NewlineDelimitedJSONEncoding"),
        ("crate::encoding", "ORCEncoding"),
        ("crate::encoding", "ONNXEncoding"),
        ("crate::schema", "UndefinedTabularSchema"),
        ("crate::schema", "TabularSchema"),
        ("crate::schema", "TimeOrderedTabularSchema"),
        ("crate::schema", "DataSchema"),
        ("crate::data_setup", "Universe"),
        ("crate::storage_setup", "LocalStorageSetup"),
        ("crate::storage_setup", "RemoteStorageSetup"),
        ("crate::storage_setup", "ReplicationStorageSetup"),
        ("crate::storage_setup", "ComputedFromLocalData"),
        ("crate::storage_setup", "StorageSetup"),
        ("crate::storage", "Storage"),
        ("crate::storage", "BigQueryStorage"),
        ("crate::storage", "SQLiteStorage"),
        ("crate::storage", "HiveTableStorage"),
        ("crate::storage", "RemoteStorage"),
        ("crate::storage", "LocalFileStorage"),
        ("crate::storage", "PostgresStorage"),
        ("crate::storage", "GitStorage"),
        ("crate::role_binding", "RoleBinding"),
        ("crate::template", "DatumTemplate"),
        ("crate::template", "IdentifierTuple"),
        ("crate::template", "RowStruct"),
        ("crate::template", "IntegerMeasure"),
        ("crate::template", "Filter"),
        ("crate::user", "User"),
        ("crate::user_group", "UserGroup"),
        ("crate::endpoints", "EndpointConfig"),
        ("crate::endpoints", "AWSConfig"),
        ("crate::endpoints", "GCPConfig"),
        ("crate::endpoints", "GiteaConfig"),
        ("crate::endpoints", "PostgresConfig"),
        ("crate::endpoints", "AlluxioConfig"),
        ("crate::endpoints", "RangerConfig"),
        ("crate::endpoints", "PrestoConfig"),
        ("crate::endpoints", "MinioConfig"),
        ("crate::template", "TrainedFloatMeasure"),
        ("crate::template", "PredictionsFromTrainedFloatMeasure"),
    ];
    scope.import("aorist_primitives", "register_concept");
    scope.import("std::convert", "TryFrom");
    scope.import("std::collections", "HashMap");
    scope.import("tracing", "debug");
    for (x, y) in &concepts {
        scope.import(x, y);
    }

    let concept_names: Vec<String> = concepts.iter().map(|(_, x)| x.to_string()).collect();
    let register = format!(
        "register_concept!(Concept, ConceptAncestry, {});",
        concept_names
            .iter()
            .map(|x| format!("{x}", x = x))
            .collect::<Vec<_>>()
            .join(", ")
    );
    scope.raw(&register);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("concepts.rs");
    fs::write(&dest_path, scope.to_string()).unwrap();
}

/// From [this comment](https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-450750547)
#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
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
    process_concepts();
    let raw_objects = read_file("basic.yaml");
    process_constraints(&raw_objects);

    let s = fs::read_to_string("programs.yaml").unwrap();
    let programs = s
        .split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| {
            let p: Result<BuildObject, _> = from_str(x);
            p
        })
        .map(|x| match x.unwrap() {
            BuildObject::Program(p) => p,
        })
        .collect::<Vec<Program>>();

    let mut scope = Scope::new();
    let mut by_uses: HashMap<String, HashMap<String, HashMap<String, Program>>> = HashMap::new();
    let mut program_uses = HashSet::new();
    let mut roots = HashSet::new();

    for x in programs.into_iter() {
        roots.insert(x.root.clone());
        program_uses.insert(x.r#use.clone());
        by_uses
            .entry(x.root.clone())
            .or_insert(HashMap::new())
            .entry(x.r#use.clone())
            .or_insert(HashMap::new())
            .entry(x.dialect.clone())
            .or_insert(x);
    }
    scope.import("aorist_primitives", "define_program");
    scope.import("crate::dialect", "Dialect");
    scope.import("aorist_primitives", "register_programs_for_constraint");
    scope.import("aorist_primitives", "register_satisfiable_constraints");
    scope.import("crate::concept", "ConceptAncestry");
    scope.import("crate::constraint", "AoristConstraint");
    scope.import("crate::constraint", "ConstraintSatisfactionBase");
    scope.import("crate::constraint", "SatisfiableConstraint");
    scope.import("crate::constraint", "AllConstraintsSatisfiability");
    scope.import("aorist_core", "ParameterTuple");
    scope.import("anyhow", "Result");
    scope.import("linked_hash_map", "LinkedHashMap");
    scope.import("textwrap", "fill");
    scope.import("textwrap", "Options");
    scope.import("hyphenation", "Language");
    scope.import("hyphenation", "Load");
    scope.import("hyphenation", "Standard");
    scope.import("crate", "*");
    for (root, constraints) in &by_uses {
        for (constraint, dialects) in constraints {
            scope.import("crate::constraint", constraint);
            scope.import("crate::constraint", &format!("Satisfy{}", constraint));

            for (dialect, program) in dialects {
                scope.import("crate::dialect", dialect);
                let mut format_strings: Vec<String> = Vec::new();
                let mut params: Vec<String> = Vec::new();
                let mut object_names: HashSet<String> = HashSet::new();
                if let Some(ref params_v) = program.args {
                    for p in params_v {
                        if let Arg::AncestorArgument(param) = p {
                            format_strings.push("'{}'".to_string());
                            params.push(param.call.clone());
                            object_names.insert(param.attaches.clone());
                        } else if let Arg::MultipleAncestorsArgument(param) = p {
                            format_strings.push("'{}'".to_string());
                            params.push(param.call.clone());
                            for obj in &param.attaches {
                                object_names.insert(obj.clone());
                            }
                        }
                    }
                }
                let mut kwargs: HashMap<String, String> = HashMap::new();
                if let Some(ref kwargs_v) = program.kwargs {
                    for (name, p) in kwargs_v.iter() {
                        if let Arg::AncestorArgument(param) = p {
                            kwargs.insert(name.clone(), param.call.clone());
                            object_names.insert(param.attaches.clone());
                        } else if let Arg::MultipleAncestorsArgument(param) = p {
                            kwargs.insert(name.clone(), param.call.clone());
                            for obj in &param.attaches {
                                object_names.insert(obj.clone());
                            }
                        }
                    }
                }

                let preamble = match &program.preamble {
                    Some(p) => p.replace("\"", "\\\""),
                    None => "".to_string(),
                };

                let define = formatdoc! {
                    "define_program!(
                        {dialect}{constraint},
                        {root}, {constraint},
                        Satisfy{constraint}, 'a, 'b, {dialect},
                        \"{preamble}\", \"{call}\",
                        |
                            uuid: Uuid,
                            concept: Concept<'a>,
                            ancestry: Arc<ConceptAncestry<'a>>,
                        | {{
                            {objects}
                            let args = vec![{params}];
                            let {mut_kw}kwargs: LinkedHashMap<String, String> =
                            LinkedHashMap::new();
                            {kwargs}
                            ParameterTuple::new(uuid, args, kwargs, {is_sql})
                        }},
                        || {{ {dialect_call} }}
                    );",
                    dialect_call = match dialect.as_str() {
                        "Python" =>
                            format!(
                                "Python::new(
                                     vec![{pip_requirements}]
                                 )",
                                pip_requirements = match &program.pip_requires {
                                    Some(v) => v
                                      .iter()
                                      .map(
                                          |x| format!("\"{}\".to_string()", x).to_string()
                                       ).collect::<Vec<String>>().join(","),
                                    None => "".into(),
                                }
                            ).to_string(),
                        _ => format!("{}{{}}", dialect),
                    },
                    is_sql=dialect == "Presto",
                    objects=object_names.iter().map(|x| {
                        format!(
                            "let {x} = match ancestry.{x}(concept.clone()) {{
                                Ok(out) => out,
                                Err(err) => panic!(
                                    \"Error encountered for constraint {{}}:\n{{}}\",
                                    \"{constraint}\",
                                    err,
                                )
                            }};",
                            constraint=constraint,
                            x=to_snake_case(x),
                        ).to_string()
                    }).collect::<Vec<String>>().join("\n"),
                    root=program.root,
                    constraint=program.r#use,
                    dialect=program.dialect,
                    preamble=preamble,
                    mut_kw=match kwargs.len() {
                        0 => "",
                        _ => "mut ",
                    },
                    call=program.call,
                    params=params.iter().map(
                        |x| format!("{x}.clone()", x=x).to_string()
                    ).collect::<Vec<String>>().join(", "),
                    kwargs=kwargs.iter().map(|(k, v)| {
                        formatdoc!(
                            "kwargs.insert(
                                 \"{k}\".to_string(),
                                 {call}.clone(),
                             );",
                             k=k, call=v,
                        ).to_string()
                    }).collect::<Vec<String>>().join("\n"),
                };
                scope.raw(&define);
            }
            let register = formatdoc! {
                "register_programs_for_constraint!(
                     {constraint}, {root}, 'a, 'b, ConceptAncestry<'a>,
                     {programs}
                );",
                constraint=constraint,
                root=root,
                programs=dialects.keys().map(
                    |k| format!("{}, {}{}", k, k, constraint)
                ).collect::<Vec<String>>().join(", "),
            };
            scope.raw(&register);
        }
    }
    let register = formatdoc! {
        "register_satisfiable_constraints!({constraints});",
        constraints=by_uses.values().map(
            |x| x.keys().map(|y| y.to_string())
        ).flatten().collect::<Vec<String>>().join(", "),
    };
    scope.raw(&register);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("programs.rs");

    fs::write(&dest_path, scope.to_string()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=basic.yaml");
    println!("cargo:rerun-if-changed=constrainables.txt");
}

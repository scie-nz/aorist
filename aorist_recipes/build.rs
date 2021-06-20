#[cfg(feature = "r")]
extern crate bindgen;

use codegen::Scope;
use indoc::formatdoc;
use inflector::cases::snakecase::to_snake_case;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
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


fn main() {

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
    scope.import("aorist_core", "Dialect");
    scope.import("aorist_primitives", "register_programs_for_constraint");
    scope.import("aorist_primitives", "register_satisfiable_constraints");
    scope.import("aorist_primitives", "TConceptEnum");
    scope.import("aorist_core", "ConceptAncestry");
    scope.import("aorist_constraint", "AoristConstraint");
    scope.import("aorist_primitives", "ConstraintSatisfactionBase");
    scope.import("aorist_core", "SatisfiableConstraint");
    scope.import("aorist_core", "SatisfiableOuterConstraint");
    scope.import("aorist_core", "ParameterTuple");
    scope.import("anyhow", "Result");
    scope.import("linked_hash_map", "LinkedHashMap");
    scope.import("textwrap", "fill");
    scope.import("textwrap", "Options");
    scope.import("hyphenation", "Language");
    scope.import("hyphenation", "Load");
    scope.import("hyphenation", "Standard");
    for (root, constraints) in &by_uses {
        scope.import("aorist_core", root);
        for (constraint, dialects) in constraints {
            scope.import("aorist_constraint", constraint);
            scope.import("aorist_constraint", &format!("Satisfy{}", constraint));

            for (dialect, program) in dialects {
                scope.import("aorist_core", dialect);
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
            //scope.raw(&register);
        }
    }
    let register = formatdoc! {
        "register_satisfiable_constraints!(Constraint, {constraints});",
        constraints=by_uses.values().map(
            |x| x.keys().map(|y| y.to_string())
        ).flatten().collect::<Vec<String>>().join(", "),
    };
    //scope.raw(&register);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("programs.rs");

    fs::write(&dest_path, scope.to_string()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=basic.yaml");
    println!("cargo:rerun-if-changed=constrainables.txt");
    println!("cargo:rustc-cfg=feature=\"build-time\"");
}

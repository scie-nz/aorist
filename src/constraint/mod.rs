use crate::concept::{AoristConcept, Concept, ConceptAncestry};
use crate::object::TAoristObject;
use aorist_primitives::{define_constraint, register_constraint, Dialect};
use maplit::hashmap;
use rustpython_parser::ast::{Expression, ExpressionType, Keyword, Located, Location, StringGroup};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct StringLiteral {
    value: String,
}
impl StringLiteral {
    pub fn new(value: String) -> Self {
        Self { value }
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

#[derive(Clone)]
enum ArgType {
    StringLiteral(Rc<StringLiteral>),
    SimpleIdentifier(String),
    Subscript(Box<ArgType>, Box<ArgType>),
    Formatted(Box<ArgType>, HashMap<String, Box<ArgType>>),
}

impl ArgType {
    pub fn expression(&self, location: Location) -> Expression {
        match &self {
            ArgType::StringLiteral(v) => {
                let value;
                if v.len() <= 60 {
                    value = StringGroup::Constant {
                        value: v.value(),
                    };
                } else {
                    let mut splits = v.value()
                        .split(",")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .into_iter();
                    let mut acc: String = splits.next().unwrap();
                    let mut values: Vec<StringGroup> = Vec::new();
                    for split in splits {
                        if acc.len() + split.len() + 1 >= 60 {
                            values.push(StringGroup::Constant { value: acc.clone() });
                            acc = "".to_string();
                        }
                        acc += ",";
                        acc += &split;
                    }
                    value = StringGroup::Joined { values };
                }
                Located {
                    location,
                    node: ExpressionType::String { value },
                }
            }
            ArgType::SimpleIdentifier(ref name) => Located {
                location,
                node: ExpressionType::Identifier { name: name.clone() },
            },
            ArgType::Formatted(box ref fmt, ref keywords) => Located {
                location,
                node: ExpressionType::Call {
                    function: Box::new(Located {
                        location,
                        node: ExpressionType::Attribute {
                            value: Box::new(fmt.expression(location)),
                            name: "format".to_string(),
                        },
                    }),
                    args: Vec::new(),
                    keywords: keywords
                        .into_iter()
                        .map(|(k, v)| Keyword {
                            name: Some(k.clone()),
                            value: v.expression(location),
                        })
                        .collect(),
                },
            },
            ArgType::Subscript(box ref a, box ref b) => Located {
                location,
                node: ExpressionType::Subscript {
                    a: Box::new(a.expression(location)),
                    b: Box::new(b.expression(location)),
                },
            },
        }
    }
}
#[derive(Clone)]
pub struct ParameterTuple {
    args: Vec<ArgType>,
    kwargs: HashMap<String, ArgType>,
}
impl ParameterTuple {
    pub fn new(args: Vec<String>, kwargs: HashMap<String, String>) -> Self {
        // TODO: should be moved into parameter tuple
        let mut literals: HashMap<String, Rc<StringLiteral>> = HashMap::new();
        Self {
            args: args
                .into_iter()
                .map(|x| {
                    ArgType::StringLiteral(
                        literals
                            .entry(x.clone())
                            .or_insert(Rc::new(StringLiteral::new(x)))
                            .clone(),
                    )
                })
                .collect(),
            kwargs: kwargs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ArgType::StringLiteral(
                            literals
                                .entry(v.clone())
                                .or_insert(Rc::new(StringLiteral::new(v)))
                                .clone(),
                        ),
                    )
                })
                .collect(),
        }
    }
    fn get_args_literals(&self, location: Location) -> Vec<Expression> {
        let args = self
            .args
            .iter()
            .map(|x| x.expression(location))
            .collect::<Vec<_>>();
        args
    }
    pub fn get_args_tuple(&self, location: Location) -> Expression {
        Located {
            location,
            node: ExpressionType::Tuple {
                elements: self.get_args_literals(location),
            },
        }
    }
    pub fn get_keyword_vector(&self, location: Location) -> Vec<Keyword> {
        self.kwargs
            .iter()
            .map(|(k, v)| Keyword {
                name: Some(k.clone()),
                value: v.expression(location),
            })
            .collect::<Vec<Keyword>>()
    }
    pub fn populate_call(&self, function: Expression, location: Location) -> Expression {
        Located {
            location,
            node: ExpressionType::Call {
                function: Box::new(function),
                args: self.get_args_literals(location),
                keywords: self.get_keyword_vector(location),
                // TODO: add keywords
            },
        }
    }
    pub fn get_args_format_string(&self) -> String {
        self.args
            .iter()
            .map(|_| "%s".to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
    fn get_call_as_format_string(&self, call: String) -> Box<StringGroup> {
        let call_format = format!("{} {}", call, self.get_args_format_string()).to_string();
        Box::new(StringGroup::Constant { value: call_format })
    }
    pub fn get_presto_query(&self, mut call: String) -> String {
        if self.args.len() > 0 {
            panic!("Do not expect self.args to be > 0 for presto queries.");
        }
        for (k, arg) in &self.kwargs {
            let fmt: String = format!("{{{}}}", k).to_string();
            if let ArgType::StringLiteral(v) = arg {
                call = call.replace(&fmt, &v.value());
            }
        }
        call
    }
    pub fn get_shell_task_command(&self, location: Location, call: String) -> Expression {
        let left = match self.args.len() {
            0 => Located {
                location,
                node: ExpressionType::String {
                    value: StringGroup::Constant { value: call },
                },
            },
            _ => {
                let args = self.get_args_tuple(location);
                let spec_str = self.get_call_as_format_string(call);
                Located {
                    location,
                    node: ExpressionType::String {
                        value: StringGroup::FormattedValue {
                            value: Box::new(args),
                            conversion: None,
                            spec: Some(spec_str),
                        },
                    },
                }
            }
        };
        if self.kwargs.len() > 0 {
            return Located {
                location,
                node: ExpressionType::Call {
                    function: Box::new(Located {
                        location,
                        node: ExpressionType::Attribute {
                            value: Box::new(left),
                            name: "format".to_string(),
                        },
                    }),
                    args: Vec::new(),
                    keywords: self.get_keyword_vector(location),
                },
            };
        }
        return left;
    }
}

pub trait TConstraint
where
    Self::Root: AoristConcept,
{
    type Root;
    fn get_root_type_name() -> String;
    fn get_required_constraint_names() -> Vec<String>;
}
pub trait ConstraintSatisfactionBase
where
    Self::RootType: AoristConcept,
    Self::ConstraintType: TConstraint<Root = Self::RootType>,
{
    type ConstraintType;
    type RootType;
}

pub trait SatisfiableConstraint: TConstraint {
    fn satisfy<'a>(
        &self,
        c: Concept<'a>,
        d: &Dialect,
        ancestry: Arc<ConceptAncestry<'a>>,
    ) -> Option<(String, String, ParameterTuple)>;

    fn satisfy_given_preference_ordering<'a>(
        &self,
        r: Concept<'a>,
        preferences: &Vec<Dialect>,
        ancestry: Arc<ConceptAncestry<'a>>,
    ) -> Result<(String, String, ParameterTuple, Dialect), String>;
}
// TODO: duplicate function, should be unified in trait
pub trait AllConstraintsSatisfiability {
    fn satisfy_given_preference_ordering<'a>(
        &self,
        c: Concept<'a>,
        preferences: &Vec<Dialect>,
        ancestry: Arc<ConceptAncestry<'a>>,
    ) -> Result<(String, String, ParameterTuple, Dialect), String>;
}

include!(concat!(env!("OUT_DIR"), "/constraints.rs"));

#[derive(Serialize, Deserialize)]
pub struct Constraint {
    #[serde(skip)]
    pub inner: Option<AoristConstraint>,
    pub name: String,
    pub root: String,
    pub requires: Option<Vec<String>>,
}
impl Constraint {
    pub fn get_uuid(&self) -> Uuid {
        if let Some(c) = &self.inner {
            return c.get_uuid();
        }
        panic!("Called get_uuid() on a Constraint struct with no inner");
    }
    pub fn get_root(&self) -> String {
        self.root.clone()
    }
    pub fn get_root_uuid(&self) -> Uuid {
        if let Some(c) = &self.inner {
            return c.get_root_uuid();
        }
        panic!("Called get_root_uuid() on a Constraint struct with no inner");
    }
    pub fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
        if let Some(c) = &self.inner {
            return c.get_downstream_constraints();
        }
        panic!("Called get_downstream_constraints() on a Constraint struct with no inner");
    }
    pub fn get_downstream_constraints_ignore_chains(&self) -> Vec<Arc<RwLock<Constraint>>> {
        if let Some(c) = &self.inner {
            return c.get_downstream_constraints_ignore_chains();
        }
        panic!("Called get_downstream_constraints() on a Constraint struct with no inner");
    }
    pub fn ingest_upstream_constraints(
        &mut self,
        upstream_constraints: Vec<Arc<RwLock<Constraint>>>,
    ) {
        if let Some(ref mut c) = &mut self.inner {
            return c.ingest_upstream_constraints(upstream_constraints);
        }
        panic!("Called ingest_upstream_constraints() on a Constraint struct with no inner");
    }
    pub fn requires_program(&self) -> bool {
        if let Some(ref c) = &self.inner {
            return c.requires_program();
        }
        panic!("Called requires_program() on a Constraint struct with no inner");
    }
    pub fn print_dag(&self) {
        for downstream_rw in self.get_downstream_constraints() {
            let downstream = downstream_rw.read().unwrap();
            println!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                self.get_name(),
                self.get_root(),
                self.get_root_uuid(),
                self.get_uuid(),
                downstream,
                downstream.get_root(),
                downstream.get_root_uuid(),
                downstream.get_uuid(),
            );
        }
        for downstream_rw in self.get_downstream_constraints() {
            let downstream = downstream_rw.read().unwrap();
            downstream.print_dag();
        }
    }
}
impl TAoristObject for Constraint {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

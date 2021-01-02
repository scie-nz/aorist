use crate::concept::{AoristConcept, Concept, ConceptAncestry};
use crate::object::TAoristObject;
use aorist_primitives::{define_constraint, register_constraint, Dialect};
use linked_hash_map::LinkedHashMap;
use maplit::hashmap;
use rustpython_parser::ast::{
    Expression, ExpressionType, Keyword, Located, Location, Statement, StatementType, StringGroup,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
pub struct StringLiteral {
    value: String,
    // TODO: replace with LinkedHashMap<Uuid, BTreeSet>
    object_uuids: LinkedHashMap<Uuid, BTreeSet<Option<String>>>,
    referenced_by: Option<Box<ArgType>>,
}

impl StringLiteral {
    pub fn new(value: String) -> Self {
        Self {
            value,
            object_uuids: LinkedHashMap::new(),
            referenced_by: None,
        }
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }
    pub fn len(&self) -> usize {
        self.value.len()
    }
    pub fn register_object(&mut self, uuid: Uuid, tag: Option<String>) {
        self.object_uuids
            .entry(uuid)
            .or_insert(BTreeSet::new())
            .insert(tag);
    }
    pub fn set_referenced_by(&mut self, obj: Box<ArgType>) {
        self.referenced_by = Some(obj);
    }
    pub fn get_object_uuids(&self) -> &LinkedHashMap<Uuid, BTreeSet<Option<String>>> {
        &self.object_uuids
    }
    pub fn is_multiline(&self) -> bool {
        self.value.contains('\n')
    }
    pub fn expression(&self, location: Location) -> Expression {
        if let Some(ref val) = self.referenced_by {
            return val.expression(location);
        }
        let value;
        if self.value.len() <= 60 || self.is_multiline() {
            value = StringGroup::Constant {
                value: self.value.clone(),
            };
        } else {
            let mut splits = self
                .value
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
            if acc.len() > 0 {
                values.push(StringGroup::Constant { value: acc.clone() });
            }
            value = StringGroup::Joined { values };
        }
        Located {
            location,
            node: ExpressionType::String { value },
        }
    }
}
pub type LiteralsMap = Arc<RwLock<HashMap<String, Arc<RwLock<StringLiteral>>>>>;

// TODO: replace HashMaps with LinkedHashMaps
#[derive(Clone)]
pub enum ArgType {
    StringLiteral(Arc<RwLock<StringLiteral>>),
    SimpleIdentifier(String),
    Subscript(Box<ArgType>, Box<ArgType>),
    Formatted(Box<ArgType>, LinkedHashMap<String, ArgType>),
    Call(Box<ArgType>, Vec<ArgType>, LinkedHashMap<String, ArgType>),
    Attribute(Box<ArgType>, String),
    List(Vec<ArgType>),
}
impl ArgType {
    pub fn register_object(&mut self, uuid: Uuid, tag: Option<String>) {
        if let ArgType::StringLiteral(ref mut s) = self {
            s.write().unwrap().register_object(uuid, tag);
        } else {
            panic!(".register_object called on non-StringLiteral ArgType.");
        }
    }
}
impl Hash for ArgType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            ArgType::StringLiteral(v) => v.read().unwrap().hash(state),
            ArgType::SimpleIdentifier(ref name) => name.hash(state),
            ArgType::Formatted(box ref fmt, ref keywords) => {
                fmt.hash(state);
                for keyword in keywords {
                    keyword.hash(state);
                }
            }
            ArgType::Subscript(box ref a, box ref b) => {
                a.hash(state);
                b.hash(state);
            }
            ArgType::Call(box ref function, ref args, ref keywords) => {
                function.hash(state);
                for arg in args.iter() {
                    arg.hash(state);
                }
                for kwarg in keywords.iter() {
                    kwarg.hash(state);
                }
            }
            ArgType::Attribute(box ref value, ref name) => {
                value.hash(state);
                name.hash(state);
            }
            ArgType::List(ref elems) => {
                for elem in elems.iter() {
                    elem.hash(state)
                }
            }
        }
    }
}

impl ArgType {
    pub fn expression(&self, location: Location) -> Expression {
        match &self {
            ArgType::StringLiteral(v) => v.read().unwrap().expression(location),
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
            ArgType::Call(box ref function, ref args, ref keywords) => Located {
                location,
                node: ExpressionType::Call {
                    function: Box::new(function.expression(location)),
                    args: args
                        .iter()
                        .map(|x| x.expression(location))
                        .collect::<Vec<_>>(),
                    keywords: keywords
                        .iter()
                        .map(|(k, v)| Keyword {
                            name: Some(k.clone()),
                            value: v.expression(location),
                        })
                        .collect::<Vec<_>>(),
                },
            },
            ArgType::Attribute(box ref value, ref name) => Located {
                location,
                node: ExpressionType::Attribute {
                    value: Box::new(value.expression(location)),
                    name: name.clone(),
                },
            },
            ArgType::List(ref elems) => Located {
                location,
                node: ExpressionType::List {
                    elements: elems
                        .iter()
                        .map(|x| x.expression(location))
                        .collect::<Vec<_>>(),
                },
            },
        }
    }
}

#[derive(Clone)]
pub enum AoristStatement {
    Assign(ArgType, ArgType),
    Expression(ArgType),
    For(ArgType, ArgType, Vec<AoristStatement>),
}
impl AoristStatement {
    pub fn statement(&self, location: Location) -> Statement {
        match &self {
            Self::Assign(target, call) => {
                let assign = StatementType::Assign {
                    targets: vec![target.expression(location)],
                    value: call.expression(location),
                };
                Located {
                    location,
                    node: assign,
                }
            }
            Self::Expression(expr) => Located {
                location,
                node: StatementType::Expression {
                    expression: expr.expression(location),
                },
            },
            Self::For(ref target, ref iter, ref body) => Located {
                location,
                node: StatementType::For {
                    is_async: false,
                    target: Box::new(target.expression(location)),
                    iter: Box::new(iter.expression(location)),
                    body: body
                        .iter()
                        .map(|x| x.statement(location))
                        .collect::<Vec<_>>(),
                    orelse: None,
                },
            },
        }
    }
}

#[derive(Clone)]
pub struct ParameterTuple {
    object_uuid: Uuid,
    args: Vec<ArgType>,
    kwargs: LinkedHashMap<String, ArgType>,
}
impl ParameterTuple {
    pub fn new(
        object_uuid: Uuid,
        args_v: Vec<String>,
        kwargs_v: LinkedHashMap<String, String>,
        literals: LiteralsMap,
    ) -> Self {
        // TODO: should be moved into parameter tuple
        let mut write = literals.write().unwrap();
        let mut args = args_v
            .into_iter()
            .map(|x| {
                ArgType::StringLiteral(
                    write
                        .entry(x.clone())
                        .or_insert(Arc::new(RwLock::new(StringLiteral::new(x))))
                        .clone(),
                )
            })
            .collect::<Vec<_>>();
        let mut kwargs = kwargs_v
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    ArgType::StringLiteral(
                        write
                            .entry(v.clone())
                            .or_insert(Arc::new(RwLock::new(StringLiteral::new(v))))
                            .clone(),
                    ),
                )
            })
            .collect::<LinkedHashMap<_, _>>();
        for arg in args.iter_mut() {
            arg.register_object(object_uuid.clone(), None);
        }
        for (k, arg) in kwargs.iter_mut() {
            arg.register_object(object_uuid.clone(), Some(k.clone()));
        }
        Self {
            object_uuid,
            args,
            kwargs,
        }
    }
    pub fn get_args(&self) -> Vec<ArgType> {
        self.args.clone()
    }
    pub fn get_kwargs(&self) -> LinkedHashMap<String, ArgType> {
        self.kwargs.clone()
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
    pub fn get_args_format_string(&self) -> String {
        self.args
            .iter()
            .map(|_| "%s".to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn get_presto_query(&self, mut call: String) -> String {
        if self.args.len() > 0 {
            panic!("Do not expect self.args to be > 0 for presto queries.");
        }
        for (k, arg) in &self.kwargs {
            let fmt: String = format!("{{{}}}", k).to_string();
            if let ArgType::StringLiteral(v) = arg {
                call = call.replace(&fmt, &v.read().unwrap().value());
            }
        }
        call
    }
    pub fn get_shell_task_command(&self, location: Location, left: Expression) -> Expression {
        // TODO: convert this to using a Literal
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
                    // TODO: args are not currently handled
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
        &mut self,
        c: Concept<'a>,
        d: &Dialect,
        ancestry: Arc<ConceptAncestry<'a>>,
        literals: LiteralsMap,
    ) -> Option<(String, String, ParameterTuple)>;

    fn satisfy_given_preference_ordering<'a>(
        &mut self,
        r: Concept<'a>,
        preferences: &Vec<Dialect>,
        ancestry: Arc<ConceptAncestry<'a>>,
        literals: LiteralsMap,
    ) -> Result<(String, String, ParameterTuple, Dialect), String>;
}
// TODO: duplicate function, should be unified in trait
pub trait AllConstraintsSatisfiability {
    fn satisfy_given_preference_ordering<'a>(
        &mut self,
        c: Concept<'a>,
        preferences: &Vec<Dialect>,
        ancestry: Arc<ConceptAncestry<'a>>,
        literals: LiteralsMap,
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

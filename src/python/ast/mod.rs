mod string_literal;

pub use string_literal::StringLiteral;

use aorist_primitives::{define_ast_node, register_ast_nodes};
use linked_hash_map::LinkedHashMap;
use num_bigint::BigInt;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyString, PyTuple};
use rustpython_parser::ast::{
    Expression, ExpressionType, ImportSymbol, Keyword, Located, Location, Number, Statement,
    StatementType, StringGroup,
};
use rustpython_parser::parser;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub type LiteralsMap = Arc<RwLock<HashMap<String, Arc<RwLock<StringLiteral>>>>>;

define_ast_node!(
    List,
    |location: Location, list: &List| Located {
        location,
        node: ExpressionType::List {
            elements: list
                .elems()
                .iter()
                .map(|x| x.expression(location))
                .collect::<Vec<_>>(),
        },
    },
    |list: &List| list.elems().clone(),
    |list: &List, py: Python, ast_module: &'a PyModule| {
        let mode = ast_module
            .call0(match list.store {
                true => "Store",
                false => "Load",
            })
            .unwrap();
        let children = list
            .elems
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module).unwrap())
            .collect::<Vec<_>>();
        let children_list = PyList::new(py, children);
        ast_module.call1("List", (children_list.as_ref(), mode))
    },
    elems: Vec<ArgType>,
    store: bool,
);

define_ast_node!(
    Dict,
    |location: Location, dict: &Dict| Located {
        location,
        node: ExpressionType::Dict {
            elements: dict
                .elems()
                .iter()
                .map(|(k, v)| {
                    (
                        Some(Located {
                            location,
                            node: ExpressionType::String {
                                value: StringGroup::Constant { value: k.clone() },
                            },
                        }),
                        v.expression(location),
                    )
                })
                .collect::<Vec<_>>(),
        },
    },
    |dict: &Dict| dict.elems().values().cloned().collect::<Vec<ArgType>>(),
    |dict: &Dict, py: Python, ast_module: &'a PyModule| {
        let keys = dict.elems.keys().map(|x| x.clone()).collect::<Vec<_>>();
        let values = dict
            .elems
            .values()
            .map(|x| x.to_python_ast_node(py, ast_module).unwrap())
            .collect::<Vec<_>>();
        let keys_list = PyList::new(py, keys);
        let values_list = PyList::new(py, values);
        ast_module.call1("Dict", (keys_list.as_ref(), values_list.as_ref()))
    },
    elems: LinkedHashMap<String, ArgType>,
);
define_ast_node!(
    Tuple,
    |location: Location, tuple: &Tuple| Located {
        location,
        node: ExpressionType::Tuple {
            elements: tuple
                .elems()
                .iter()
                .map(|x| x.expression(location))
                .collect::<Vec<_>>(),
        },
    },
    |tuple: &Tuple| tuple.elems().iter().cloned().collect::<Vec<ArgType>>(),
    |tuple: &Tuple, py: Python, ast_module: &'a PyModule| {
        let mode = ast_module
            .call0(match tuple.store {
                true => "Store",
                false => "Load",
            })
            .unwrap();
        let children = tuple
            .elems
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module).unwrap())
            .collect::<Vec<_>>();
        let children_list = PyList::new(py, children);
        ast_module.call1("Tuple", (children_list.as_ref(), mode))
    },
    elems: Vec<ArgType>,
    store: bool,
);

define_ast_node!(
    Attribute,
    |location: Location, attr: &Attribute| Located {
        location,
        node: ExpressionType::Attribute {
            value: Box::new(attr.value().expression(location)),
            name: attr.name().clone(),
        },
    },
    |attribute: &Attribute| vec![attribute.value().clone()],
    |attribute: &Attribute, py: Python, ast_module: &'a PyModule| {
        let mode = ast_module
            .call0(match attribute.store {
                true => "Store",
                false => "Load",
            })
            .unwrap();
        let val_ast = attribute.value.to_python_ast_node(py, ast_module)?;
        let name_ast = PyString::new(py, &attribute.name);
        ast_module.call1("Attribute", (val_ast, name_ast.as_ref(), mode))
    },
    value: ArgType,
    name: String,
    store: bool,
);

define_ast_node!(
    Call,
    |location: Location, call: &Call| Located {
        location,
        node: ExpressionType::Call {
            function: Box::new(call.function().expression(location)),
            args: call
                .args()
                .iter()
                .map(|x| x.expression(location))
                .collect::<Vec<_>>(),
            keywords: call
                .keywords()
                .iter()
                .map(|(k, v)| Keyword {
                    name: Some(k.clone()),
                    value: v.expression(location),
                })
                .collect::<Vec<_>>(),
        },
    },
    |call: &Call| {
        let mut v = vec![call.function().clone()];
        for arg in call.args() {
            v.push(arg.clone());
        }
        for kw in call.keywords().values() {
            v.push(kw.clone());
        }
        v
    },
    |call: &Call, py: Python, ast_module: &'a PyModule| {
        let args = call
            .args
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module).unwrap())
            .collect::<Vec<_>>();
        let kwargs = call
            .keywords
            .iter()
            .map(|(k, v)| {
                ast_module
                    .call1(
                        "keyword",
                        PyTuple::new(
                            py,
                            &vec![
                                PyString::new(py, k).as_ref(),
                                v.to_python_ast_node(py, ast_module).unwrap(),
                            ],
                        ),
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let function = call.function.to_python_ast_node(py, ast_module)?;
        ast_module.call1("Call", (function, args, kwargs))
    },
    function: ArgType,
    args: Vec<ArgType>,
    keywords: LinkedHashMap<String, ArgType>,
);

define_ast_node!(
    Formatted,
    |location: Location, formatted: &Formatted| Located {
        location,
        node: ExpressionType::Call {
            function: Box::new(Located {
                location,
                node: ExpressionType::Attribute {
                    value: Box::new(formatted.fmt().expression(location)),
                    name: "format".to_string(),
                },
            }),
            args: Vec::new(),
            keywords: formatted
                .keywords()
                .iter()
                .map(|(k, v)| Keyword {
                    name: Some(k.clone()),
                    value: v.expression(location),
                })
                .collect(),
        },
    },
    |formatted: &Formatted| {
        let mut v = vec![formatted.fmt().clone()];
        for kw in formatted.keywords().values() {
            v.push(kw.clone());
        }
        v
    },
    |formatted: &Formatted, py: Python, ast_module: &'a PyModule| {
        let format_fn = ast_module.call1(
            "Attribute",
            (
                formatted.fmt.to_python_ast_node(py, ast_module)?,
                PyString::new(py, "format").as_ref(),
            ),
        )?;
        let kwargs = formatted
            .keywords
            .iter()
            .map(|(k, v)| {
                ast_module
                    .call1(
                        "keyword",
                        PyTuple::new(
                            py,
                            &vec![
                                PyString::new(py, k).as_ref(),
                                v.to_python_ast_node(py, ast_module).unwrap(),
                            ],
                        ),
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let args: Vec<String> = Vec::new();
        ast_module.call1(
            "Call",
            (
                format_fn,
                PyList::new(py, args).as_ref(),
                PyList::new(py, kwargs).as_ref(),
            ),
        )
    },
    fmt: ArgType,
    keywords: LinkedHashMap<String, ArgType>,
);
define_ast_node!(
    Subscript,
    |location: Location, subscript: &Subscript| Located {
        location,
        node: ExpressionType::Subscript {
            a: Box::new(subscript.a().expression(location)),
            b: Box::new(subscript.b().expression(location)),
        },
    },
    |subscript: &Subscript| vec![subscript.a().clone(), subscript.b().clone()],
    |subscript: &Subscript, py: Python, ast_module: &'a PyModule| {
        let mode = ast_module
            .call0(match subscript.store {
                true => "Store",
                false => "Load",
            })
            .unwrap();
        let b_node = subscript.b.to_python_ast_node(py, ast_module)?;
        let idx = ast_module.call1("Index", (b_node,))?;
        let value = subscript.a.to_python_ast_node(py, ast_module)?;
        ast_module.call1("Subscript", (value, idx, mode))
    },
    a: ArgType,
    b: ArgType,
    store: bool,
);
define_ast_node!(
    SimpleIdentifier,
    |location: Location, ident: &SimpleIdentifier| Located {
        location,
        node: ExpressionType::Identifier {
            name: ident.name().clone()
        },
    },
    |_| Vec::new(),
    |simple_identifier: &SimpleIdentifier, py: Python, ast_module: &'a PyModule| {
        ast_module.call1(
            "Name",
            (PyString::new(py, &simple_identifier.name).as_ref(),),
        )
    },
    name: String,
);
define_ast_node!(
    BooleanLiteral,
    |location: Location, x: &BooleanLiteral| Located {
        location,
        node: match x.val {
            true => ExpressionType::True,
            false => ExpressionType::False,
        }
    },
    |_| Vec::new(),
    |lit: &BooleanLiteral, _py: Python, ast_module: &'a PyModule| {
        ast_module.call1("Constant", (lit.val,))
    },
    val: bool,
);
define_ast_node!(
    BigIntLiteral,
    |location: Location, ident: &BigIntLiteral| Located {
        location,
        node: ExpressionType::Number {
            value: Number::Integer {
                value: ident.val.clone()
            }
        }
    },
    |_| Vec::new(),
    |lit: &BigIntLiteral, _py: Python, ast_module: &'a PyModule| {
        let val: i64 = (*lit.val.to_u32_digits().1.get(0).unwrap()).into();
        ast_module.call1("Constant", (val,))
    },
    // TODO: deprecate use of BigInt when removing rustpython
    val: BigInt,
);
define_ast_node!(
    PythonNone,
    |location: Location, _ident: &PythonNone| Located {
        location,
        node: ExpressionType::None,
    },
    |_| Vec::new(),
    |_, py: Python, ast_module: &'a PyModule| {
        ast_module.call1("Constant", (py.None().as_ref(py),))
    },
);

register_ast_nodes!(
    ArgType,
    StringLiteral,
    SimpleIdentifier,
    Subscript,
    Formatted,
    Call,
    Attribute,
    List,
    Dict,
    Tuple,
    BooleanLiteral,
    BigIntLiteral,
    PythonNone,
);
impl PartialEq for ArgType {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (ArgType::StringLiteral(v1), ArgType::StringLiteral(v2)) => {
                v1.read().unwrap().eq(&v2.read().unwrap())
            }
            (ArgType::SimpleIdentifier(v1), ArgType::SimpleIdentifier(v2)) => {
                v1.read().unwrap().eq(&v2.read().unwrap())
            }
            (ArgType::Subscript(v1), ArgType::Subscript(v2)) => {
                v1.read().unwrap().eq(&v2.read().unwrap())
            }
            (ArgType::Formatted(v1), ArgType::Formatted(v2)) => {
                v1.read().unwrap().eq(&v2.read().unwrap())
            }
            (ArgType::Call(v1), ArgType::Call(v2)) => v1.read().unwrap().eq(&v2.read().unwrap()),
            (ArgType::Attribute(v1), ArgType::Attribute(v2)) => {
                v1.read().unwrap().eq(&v2.read().unwrap())
            }
            (ArgType::List(v1), ArgType::List(v2)) => v1.read().unwrap().eq(&v2.read().unwrap()),
            (ArgType::Dict(v1), ArgType::Dict(v2)) => v1.read().unwrap().eq(&v2.read().unwrap()),
            (ArgType::Tuple(v1), ArgType::Tuple(v2)) => v1.read().unwrap().eq(&v2.read().unwrap()),
            (ArgType::BooleanLiteral(v1), ArgType::BooleanLiteral(v2)) => {
                v1.read().unwrap().eq(&v2.read().unwrap())
            }
            (ArgType::PythonNone(_), ArgType::PythonNone(_)) => true,
            (ArgType::BigIntLiteral(v1), ArgType::BigIntLiteral(v2)) => {
                v1.read().unwrap().eq(&v2.read().unwrap())
            }
            _ => {
                if self.name() == other.name() {
                    panic!(format!("PartialEq not implemented for {}", self.name()))
                }
                false
            }
        }
    }
}
impl Eq for ArgType {}
impl Hash for ArgType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            ArgType::StringLiteral(v) => v.read().unwrap().hash(state),
            ArgType::SimpleIdentifier(ref x) => x.read().unwrap().hash(state),
            ArgType::Subscript(ref x) => x.read().unwrap().hash(state),
            ArgType::Formatted(ref x) => x.read().unwrap().hash(state),
            ArgType::Call(ref x) => x.read().unwrap().hash(state),
            ArgType::Attribute(ref attr) => attr.read().unwrap().hash(state),
            ArgType::List(ref list) => list.read().unwrap().hash(state),
            ArgType::Dict(ref dict) => dict.read().unwrap().hash(state),
            ArgType::Tuple(ref tuple) => tuple.read().unwrap().hash(state),
            ArgType::BooleanLiteral(ref l) => l.read().unwrap().hash(state),
            ArgType::BigIntLiteral(ref l) => l.read().unwrap().hash(state),
            ArgType::PythonNone(ref l) => l.read().unwrap().hash(state),
        }
    }
}

impl ArgType {
    pub fn expression(&self, location: Location) -> Expression {
        match &self {
            ArgType::StringLiteral(v) => v.read().unwrap().expression(location),
            ArgType::SimpleIdentifier(ref x) => x.read().unwrap().expression(location),
            ArgType::Subscript(ref x) => x.read().unwrap().expression(location),
            ArgType::Formatted(ref x) => x.read().unwrap().expression(location),
            ArgType::Call(ref x) => x.read().unwrap().expression(location),
            ArgType::Attribute(ref attr) => attr.read().unwrap().expression(location),
            ArgType::List(ref list) => list.read().unwrap().expression(location),
            ArgType::Dict(ref dict) => dict.read().unwrap().expression(location),
            ArgType::Tuple(ref tuple) => tuple.read().unwrap().expression(location),
            ArgType::BooleanLiteral(ref l) => l.read().unwrap().expression(location),
            ArgType::BigIntLiteral(ref l) => l.read().unwrap().expression(location),
            ArgType::PythonNone(ref l) => l.read().unwrap().expression(location),
        }
    }
}

pub struct AoristImportSymbol(pub ImportSymbol);
impl Clone for AoristImportSymbol {
    fn clone(&self) -> Self {
        Self(ImportSymbol {
            symbol: self.0.symbol.clone(),
            alias: self.0.alias.clone(),
        })
    }
}
impl PartialEq for AoristImportSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.0.symbol == other.0.symbol && self.0.alias == other.0.alias
    }
}
impl Eq for AoristImportSymbol {}
impl PartialOrd for AoristImportSymbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.0.symbol.partial_cmp(&other.0.symbol);
        match cmp {
            Some(Ordering::Equal) => self.0.alias.partial_cmp(&other.0.alias),
            _ => cmp,
        }
    }
}
impl Ord for AoristImportSymbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Preamble {
    pub imports: Vec<AoristImportSymbol>,
    pub from_imports: BTreeMap<Option<String>, Vec<AoristImportSymbol>>,
    statement: Statement,
}
impl Preamble {
    pub fn new(body: String) -> Preamble {
        let program = parser::parse_program(&body).unwrap();

        let mut imports = Vec::new();
        let mut from_imports = BTreeMap::new();
        let mut others = Vec::new();

        for statement in program.statements {
            if let StatementType::Import { names } = statement.node {
                for name in names {
                    imports.push(AoristImportSymbol(name));
                }
            } else if let StatementType::ImportFrom { module, names, .. } = statement.node {
                for name in names {
                    from_imports
                        .entry(module.clone())
                        .or_insert(Vec::new())
                        .push(AoristImportSymbol(name));
                }
            } else {
                others.push(statement);
            }
        }
        assert_eq!(others.len(), 1);
        Self {
            imports,
            from_imports,
            statement: others.into_iter().next().unwrap(),
        }
    }
    pub fn statement(self) -> Statement {
        self.statement
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Import {
    ModuleImport(String),
    FromImport(String, String),
}
impl Import {
    pub fn statement(&self, location: Location) -> Statement {
        match &self {
            Self::ModuleImport(ref module) => Located {
                location,
                node: StatementType::Import {
                    names: vec![ImportSymbol {
                        symbol: module.clone(),
                        alias: None,
                    }],
                },
            },
            Self::FromImport(ref module, ref name) => Located {
                location,
                node: StatementType::ImportFrom {
                    level: 0,
                    module: Some(module.clone()),
                    names: vec![ImportSymbol {
                        symbol: name.clone(),
                        alias: None,
                    }],
                },
            },
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum AoristStatement {
    Assign(ArgType, ArgType),
    Expression(ArgType),
    For(ArgType, ArgType, Vec<AoristStatement>),
    Import(Import),
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
            Self::Import(import) => import.statement(location),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParameterTuple {
    args: Vec<ArgType>,
    kwargs: LinkedHashMap<String, ArgType>,
}
pub type ParameterTupleDedupKey = (usize, Vec<String>);
impl ParameterTuple {
    pub fn populate_python_dict(&self, dict: &mut LinkedHashMap<String, ArgType>) {
        if self.args.len() > 0 {
            dict.insert(
                "args".to_string(),
                ArgType::List(List::new_wrapped(self.args.clone(), false)),
            );
        }
        for (key, val) in &self.kwargs {
            dict.insert(key.clone(), val.clone());
        }
    }
    pub fn get_dedup_key(&self) -> ParameterTupleDedupKey {
        (self.args.len(), self.kwargs.keys().cloned().collect())
    }
    pub fn new(
        object_uuid: Uuid,
        args_v: Vec<String>,
        kwargs_v: LinkedHashMap<String, String>,
        literals: LiteralsMap,
    ) -> Self {
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
        Self { args, kwargs }
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

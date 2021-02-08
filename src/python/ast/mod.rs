mod string_literal;

pub use string_literal::StringLiteral;

use aorist_primitives::{define_ast_node, register_ast_nodes};
use linked_hash_map::LinkedHashMap;
use num_bigint::BigInt;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyString, PyTuple};
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub type LiteralsMap = Arc<RwLock<HashMap<String, Arc<RwLock<StringLiteral>>>>>;

pub trait TAssignmentTarget
where
    Self: Sized,
{
    fn as_assignment_target(&self) -> Self;
    fn as_wrapped_assignment_target(&self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self.as_assignment_target()))
    }
}

define_ast_node!(
    List,
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
impl TAssignmentTarget for List {
    fn as_assignment_target(&self) -> Self {
        assert!(self.owner.is_none());
        Self {
            elems: self.elems.clone(),
            store: true,
            owner: None,
        }
    }
}

define_ast_node!(
    Dict,
    |dict: &Dict| dict.elems().values().cloned().collect::<Vec<ArgType>>(),
    |dict: &Dict, py: Python, ast_module: &'a PyModule| {
        let keys = dict
            .elems
            .keys()
            .map(|x| {
                StringLiteral::new(x.clone())
                    .to_python_ast_node(py, ast_module)
                    .unwrap()
            })
            .collect::<Vec<_>>();
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
impl TAssignmentTarget for Tuple {
    fn as_assignment_target(&self) -> Self {
        assert!(self.owner.is_none());
        Self {
            elems: self.elems.clone(),
            store: true,
            owner: None,
        }
    }
}

define_ast_node!(
    Attribute,
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
impl TAssignmentTarget for Attribute {
    fn as_assignment_target(&self) -> Self {
        assert!(self.owner.is_none());
        Self {
            value: self.value.clone(),
            name: self.name.clone(),
            store: true,
            owner: None,
        }
    }
}

define_ast_node!(
    Call,
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
impl TAssignmentTarget for Subscript {
    fn as_assignment_target(&self) -> Self {
        assert!(self.owner.is_none());
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            store: true,
            owner: None,
        }
    }
}
define_ast_node!(
    SimpleIdentifier,
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
    |_| Vec::new(),
    |lit: &BooleanLiteral, _py: Python, ast_module: &'a PyModule| {
        ast_module.call1("Constant", (lit.val,))
    },
    val: bool,
);
define_ast_node!(
    BigIntLiteral,
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

pub struct Preamble<'a> {
    pub imports: Vec<(String, Option<String>)>,
    pub from_imports: Vec<(String, String, Option<String>)>,
    pub body: Vec<&'a PyAny>,
}
impl<'a> Preamble<'a> {
    pub fn new(body: String, py: Python<'a>) -> Preamble {
        let helpers = PyModule::from_code(
            py,
            r#"
import ast

def build_preamble(body):
    module = ast.parse(body)

    imports = []
    from_imports = []
    other = []

    for elem in module.body:
        if isinstance(elem, ast.Import):
            for name in elem.names:
                imports += [(name.name, name.asname)]
        elif isinstance(elem, ast.ImportFrom):
            for name in elem.names:
                from_imports += [(elem.module, name.name, name.asname)]
        else:
            other += [elem]

    return imports, from_imports, other
        "#,
            "helpers.py",
            "helpers",
        )
        .unwrap();

        let tpl: &PyTuple = helpers
            .call1("build_preamble", (body,))
            .unwrap()
            .downcast()
            .unwrap();

        let imports_list: &PyList = tpl.get_item(0).extract().unwrap();
        let imports: Vec<(String, Option<String>)> = imports_list
            .iter()
            .map(|x| {
                let tpl: &PyTuple = x.extract().unwrap();
                let name: String = tpl
                    .get_item(0)
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let alias = tpl.get_item(1);
                let asname: Option<String> = match alias.is_none() {
                    true => None,
                    false => Some(
                        alias
                            .extract::<&PyString>()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    ),
                };
                if asname.is_some() {
                    panic!("Aliased imports not supported yet.");
                }
                (name, asname)
            })
            .collect();

        let from_imports_list: &PyList = tpl.get_item(1).extract().unwrap();
        let from_imports: Vec<(String, String, Option<String>)> = from_imports_list
            .iter()
            .map(|x| {
                let tpl: &PyTuple = x.extract().unwrap();
                let module: String = tpl
                    .get_item(0)
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let name: String = tpl
                    .get_item(1)
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let alias = tpl.get_item(2);
                let asname: Option<String> = match alias.is_none() {
                    true => None,
                    false => Some(
                        alias
                            .extract::<&PyString>()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    ),
                };
                if asname.is_some() {
                    panic!("Aliased imports not supported yet.");
                }
                (module, name, asname)
            })
            .collect();

        let body_no_imports: &PyList = tpl.get_item(2).extract().unwrap();
        Self {
            imports,
            from_imports,
            body: body_no_imports.iter().map(|x| x.clone()).collect(),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Import {
    ModuleImport(String),
    FromImport(String, String),
}
impl Import {
    pub fn to_python_ast_node<'a>(
        &self,
        py: Python,
        ast_module: &'a PyModule,
    ) -> PyResult<&'a PyAny> {
        match &self {
            Self::ModuleImport(ref module) => {
                let names = PyList::new(py, vec![module]);
                ast_module.call1("Import", (names.as_ref(),))
            }
            Self::FromImport(ref module, ref name) => {
                let names = PyList::new(
                    py,
                    vec![SimpleIdentifier::new(name.clone()).to_python_ast_node(py, ast_module)?],
                );
                ast_module.call1("ImportFrom", (module, names.as_ref(), 0))
            }
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
    pub fn to_python_ast_node<'a>(
        &self,
        py: Python,
        ast_module: &'a PyModule,
    ) -> PyResult<&'a PyAny> {
        match &self {
            Self::Assign(ref target, ref call) => {
                let assign_target = match target {
                    ArgType::Subscript(ref x) => {
                        ArgType::Subscript(x.read().unwrap().as_wrapped_assignment_target())
                    }
                    ArgType::Attribute(ref x) => {
                        ArgType::Attribute(x.read().unwrap().as_wrapped_assignment_target())
                    }
                    ArgType::List(ref x) => {
                        ArgType::List(x.read().unwrap().as_wrapped_assignment_target())
                    }
                    ArgType::Tuple(ref x) => {
                        ArgType::Tuple(x.read().unwrap().as_wrapped_assignment_target())
                    }
                    ArgType::SimpleIdentifier(_) => target.clone(),
                    _ => panic!("Assignment target not supported."),
                };
                let targets =
                    PyList::new(py, vec![assign_target.to_python_ast_node(py, ast_module)?]);
                ast_module.call1(
                    "Assign",
                    (targets, call.to_python_ast_node(py, ast_module)?),
                )
            }
            Self::Expression(ref expr) => {
                ast_module.call1("Expr", (expr.to_python_ast_node(py, ast_module)?,))
            }
            Self::For(ref target, ref iter, ref body) => {
                let body_ast = body
                    .iter()
                    .map(|x| x.to_python_ast_node(py, ast_module).unwrap())
                    .collect::<Vec<_>>();
                let body_list = PyList::new(py, body_ast);
                let empty_vec: Vec<String> = Vec::new();
                let empty_list = PyList::new(py, empty_vec);
                ast_module.call1(
                    "For",
                    (
                        target.to_python_ast_node(py, ast_module)?,
                        iter.to_python_ast_node(py, ast_module)?,
                        body_list.as_ref(),
                        empty_list.as_ref(),
                    ),
                )
            }
            Self::Import(import) => import.to_python_ast_node(py, ast_module),
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
                        .or_insert(StringLiteral::new_wrapped(x))
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
                            .or_insert(StringLiteral::new_wrapped(v))
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
}

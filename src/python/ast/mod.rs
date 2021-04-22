mod assignment_target;
mod bash_python_task;
mod constant_python_task;
mod import;
mod native_python_task;
mod preamble;
mod presto_python_task;
mod python_subprocess_task;
mod r_python_task;
mod string_literal;

pub use assignment_target::TAssignmentTarget;
pub use bash_python_task::BashPythonTask;
pub use constant_python_task::ConstantPythonTask;
pub use import::Import;
pub use native_python_task::NativePythonTask;
pub use preamble::Preamble;
pub use presto_python_task::PrestoPythonTask;
pub use r_python_task::RPythonTask;
pub use string_literal::StringLiteral;

use crate::constraint_state::AncestorRecord;
use aorist_derive::Optimizable;
use aorist_primitives::{define_ast_node, register_ast_nodes};
use extendr_api::prelude::*;
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyString, PyTuple};
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

define_ast_node!(
    ImportNode,
    |import: &ImportNode| vec![import.inner.clone()],
    |import: &ImportNode, py: Python, ast_module: &'a PyModule, depth: usize| {
        import.to_python_ast_node(py, ast_module, depth)
    },
    |import: &ImportNode, depth: usize| {
        call!("call", "library", import.inner.to_r_ast_node(depth)).unwrap()
    },
    inner: AST,
);

define_ast_node!(
    ForLoop,
    |for_loop: &ForLoop| vec![for_loop.target.clone(), for_loop.iter.clone()]
        .into_iter()
        .chain(for_loop.body.clone().into_iter())
        .collect(),
    |for_loop: &ForLoop, py: Python, ast_module: &'a PyModule, depth: usize| {
        let body_ast = for_loop
            .body
            .iter()
            .map(|x| match &x {
                AST::Assignment(_) | AST::Expression(_) => x,
                _ => panic!("AST node of type {} found in for loop body", x.name()),
            })
            .map(|x| x.to_python_ast_node(py, ast_module, depth + 1).unwrap())
            .collect::<Vec<_>>();
        let body_list = PyList::new(py, body_ast);
        let empty_vec: Vec<String> = Vec::new();
        let empty_list = PyList::new(py, empty_vec);
        ast_module.call1(
            "For",
            (
                for_loop.target.to_python_ast_node(py, ast_module, depth)?,
                for_loop.iter.to_python_ast_node(py, ast_module, depth)?,
                body_list.as_ref(),
                empty_list.as_ref(),
            ),
        )
    },
    |for_loop: &ForLoop, depth: usize| {
        let pairlist = for_loop
            .body
            .iter()
            .map(|x| x.to_r_ast_node(depth))
            .collect::<Vec<_>>();

        call!(
            "for",
            for_loop.target.to_r_ast_node(depth),
            for_loop.iter.to_r_ast_node(depth),
            call!("{", pairlist).unwrap()
        )
        .unwrap()
    },
    target: AST,
    iter: AST,
    body: Vec<AST>,
);
define_ast_node!(
    Assignment,
    |assign: &Assignment| vec![assign.target.clone(), assign.call.clone()],
    |assign: &Assignment, py: Python, ast_module: &'a PyModule, depth: usize| {
        let assign_target = match assign.target {
            AST::Subscript(ref x) => {
                AST::Subscript(x.read().unwrap().as_wrapped_assignment_target())
            }
            AST::Attribute(ref x) => {
                AST::Attribute(x.read().unwrap().as_wrapped_assignment_target())
            }
            AST::List(ref x) => AST::List(x.read().unwrap().as_wrapped_assignment_target()),
            AST::Tuple(ref x) => AST::Tuple(x.read().unwrap().as_wrapped_assignment_target()),
            AST::SimpleIdentifier(_) => assign.target.clone(),
            _ => panic!("Assignment target not supported."),
        };
        let targets = PyList::new(
            py,
            vec![assign_target.to_python_ast_node(py, ast_module, depth)?],
        );
        ast_module.call1(
            "Assign",
            (
                targets,
                assign.call.to_python_ast_node(py, ast_module, depth)?,
            ),
        )
    },
    |assign: &Assignment, depth: usize| {
        r!(Lang(&[
            r!(Symbol("<-")),
            assign.target.to_r_ast_node(depth),
            assign.call.to_r_ast_node(depth)
        ]))
    },
    target: AST,
    call: AST,
);
define_ast_node!(
    Expression,
    |expr: &Expression| vec![expr.inner.clone()],
    |expr: &Expression, py: Python, ast_module: &'a PyModule, depth: usize| {
        ast_module.call1(
            "Expr",
            (expr.inner.to_python_ast_node(py, ast_module, depth)?,),
        )
    },
    |expr: &Expression, depth: usize| { expr.inner.to_r_ast_node(depth) },
    inner: AST,
);
define_ast_node!(
    Add,
    |_node: &Add| vec![],
    |_node: &Add, _py: Python, ast_module: &'a PyModule, _depth: usize| { ast_module.call0("Add") },
    |_add: &Add, _depth: usize| { panic!("Should not call to_r_ast_node on Add objects directly") },
);
define_ast_node!(
    BinOp,
    |node: &BinOp| vec![node.left.clone(), node.right.clone()],
    |node: &BinOp, py: Python, ast_module: &'a PyModule, depth: usize| {
        ast_module.call1(
            "BinOp",
            (
                node.left.to_python_ast_node(py, ast_module, depth)?,
                node.op.to_python_ast_node(py, ast_module, depth)?,
                node.right.to_python_ast_node(py, ast_module, depth)?,
            ),
        )
    },
    |binop: &BinOp, depth: usize| {
        let op_str = match binop.op {
            AST::Add(_) => "+",
            _ => panic!("AST node not supported as R operator"),
        };
        call!(
            "call",
            op_str,
            binop.left.to_r_ast_node(depth),
            binop.right.to_r_ast_node(depth)
        )
        .unwrap()
    },
    left: AST,
    op: AST,
    right: AST,
);
define_ast_node!(
    List,
    |list: &List| list.elems().clone(),
    |list: &List, py: Python, ast_module: &'a PyModule, depth: usize| {
        let mode = ast_module
            .call0(match list.store {
                true => "Store",
                false => "Load",
            })
            .unwrap();
        let children = list
            .elems
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module, depth).unwrap())
            .collect::<Vec<_>>();
        let children_list = PyList::new(py, children);
        ast_module.call1("List", (children_list.as_ref(), mode))
    },
    |list: &List, depth: usize| {
        let elems = list
            .elems
            .iter()
            .map(|x| x.to_r_ast_node(depth))
            .collect::<Vec<_>>();
        r!(List(&elems))
    },
    elems: Vec<AST>,
    store: bool,
);
impl TAssignmentTarget for List {
    fn as_assignment_target(&self) -> Self {
        Self {
            elems: self.elems.clone(),
            store: true,
            ancestors: self.ancestors.clone(),
        }
    }
}

define_ast_node!(
    Dict,
    |dict: &Dict| dict.elems().values().cloned().collect::<Vec<AST>>(),
    |dict: &Dict, py: Python, ast_module: &'a PyModule, depth: usize| {
        let keys = dict
            .elems
            .keys()
            .map(|x| {
                StringLiteral::new(x.clone(), false)
                    .to_python_ast_node(py, ast_module, depth + 1)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let values = dict
            .elems
            .values()
            .map(|x| x.to_python_ast_node(py, ast_module, depth + 1).unwrap())
            .collect::<Vec<_>>();
        let keys_list = PyList::new(py, keys);
        let values_list = PyList::new(py, values);
        ast_module.call1("Dict", (keys_list.as_ref(), values_list.as_ref()))
    },
    |dict: &Dict, depth: usize| {
        call!(
            "list",
            Pairlist {
                names_and_values: dict
                    .elems
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_r_ast_node(depth)))
                    .collect::<Vec<(String, Robj)>>()
            }
        )
        .unwrap()
    },
    elems: LinkedHashMap<String, AST>,
);
impl Dict {
    pub fn replace_elem(&mut self, key: String, elem: AST) {
        self.elems.insert(key, elem);
    }
}
define_ast_node!(
    Tuple,
    |tuple: &Tuple| tuple.elems().iter().cloned().collect::<Vec<AST>>(),
    |tuple: &Tuple, py: Python, ast_module: &'a PyModule, depth: usize| {
        let mode = ast_module
            .call0(match tuple.store {
                true => "Store",
                false => "Load",
            })
            .unwrap();
        let children = tuple
            .elems
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module, depth + 1).unwrap())
            .collect::<Vec<_>>();
        let children_list = PyList::new(py, children);
        ast_module.call1("Tuple", (children_list.as_ref(), mode))
    },
    |_tuple: &Tuple, _depth: usize| { panic!("No R correspondent for Tuple nodes") },
    elems: Vec<AST>,
    store: bool,
);
impl TAssignmentTarget for Tuple {
    fn as_assignment_target(&self) -> Self {
        Self {
            elems: self.elems.clone(),
            store: true,
            ancestors: self.ancestors.clone(),
        }
    }
}

define_ast_node!(
    Attribute,
    |attribute: &Attribute| vec![attribute.value().clone()],
    |attribute: &Attribute, py: Python, ast_module: &'a PyModule, depth: usize| {
        let mode = ast_module
            .call0(match attribute.store {
                true => "Store",
                false => "Load",
            })
            .unwrap();
        let val_ast = attribute.value.to_python_ast_node(py, ast_module, depth)?;
        let name_ast = PyString::new(py, &attribute.name);
        ast_module.call1("Attribute", (val_ast, name_ast.as_ref(), mode))
    },
    |_attribute: &Attribute, _depth: usize| { panic!("No R correspondent for Attribute nodes") },
    value: AST,
    name: String,
    store: bool,
);
impl TAssignmentTarget for Attribute {
    fn as_assignment_target(&self) -> Self {
        Self {
            value: self.value.clone(),
            name: self.name.clone(),
            store: true,
            ancestors: self.ancestors.clone(),
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
    |call: &Call, py: Python, ast_module: &'a PyModule, depth: usize| {
        let args = call
            .args
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module, depth + 1).unwrap())
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
                                v.to_python_ast_node(py, ast_module, depth + 1).unwrap(),
                            ],
                        ),
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let function = call.function.to_python_ast_node(py, ast_module, depth)?;
        ast_module.call1("Call", (function, args, kwargs))
    },
    |call: &Call, depth: usize| {
        assert_eq!(call.args.len(), 0);
        let mut args = call.keywords.clone();
        args.insert("name".to_string(), call.function.clone());
        let arglist: Pairlist<Vec<(String, Robj)>> = Pairlist {
            names_and_values: args
                .iter()
                .map(|(k, v)| (k.clone(), v.to_r_ast_node(depth)))
                .collect(),
        };
        call!("do.call", "call", arglist).unwrap()
    },
    function: AST,
    args: Vec<AST>,
    keywords: LinkedHashMap<String, AST>,
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
    |formatted: &Formatted, py: Python, ast_module: &'a PyModule, depth: usize| {
        let format_fn = ast_module.call1(
            "Attribute",
            (
                formatted.fmt.to_python_ast_node(py, ast_module, depth)?,
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
                                v.to_python_ast_node(py, ast_module, depth + 1).unwrap(),
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
    |formatted: &Formatted, depth: usize| {
        let fmt_node = formatted.fmt.to_r_ast_node(depth);
        let arglist = extendr_api::prelude::Pairlist {
            names_and_values: formatted
                .keywords
                .iter()
                .map(|(k, v)| (k.clone(), v.to_r_ast_node(depth)))
                .collect::<Vec<_>>(),
        };
        call!("call", "glue::glue", fmt_node, arglist).unwrap()
    },
    fmt: AST,
    keywords: LinkedHashMap<String, AST>,
);
define_ast_node!(
    Subscript,
    |subscript: &Subscript| vec![subscript.a().clone(), subscript.b().clone()],
    |subscript: &Subscript, py: Python, ast_module: &'a PyModule, depth: usize| {
        let mode = ast_module
            .call0(match subscript.store {
                true => "Store",
                false => "Load",
            })
            .unwrap();
        let b_node = subscript.b.to_python_ast_node(py, ast_module, depth + 1)?;
        let idx = ast_module.call1("Index", (b_node,))?;
        let value = subscript.a.to_python_ast_node(py, ast_module, depth + 1)?;
        ast_module.call1("Subscript", (value, idx, mode))
    },
    |subscript: &Subscript, depth: usize| {
        let a_node = subscript.a.to_r_ast_node(depth);
        let b_node = subscript.b.to_r_ast_node(depth);
        call!("call", "[[", a_node, b_node).unwrap()
    },
    a: AST,
    b: AST,
    store: bool,
);
impl TAssignmentTarget for Subscript {
    fn as_assignment_target(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            store: true,
            ancestors: self.ancestors.clone(),
        }
    }
}
define_ast_node!(
    SimpleIdentifier,
    |_| Vec::new(),
    |simple_identifier: &SimpleIdentifier, py: Python, ast_module: &'a PyModule, _depth: usize| {
        ast_module.call1(
            "Name",
            (PyString::new(py, &simple_identifier.name).as_ref(),),
        )
    },
    |simple_identifier: &SimpleIdentifier, _depth: usize| {
        //let call = format!("call(\"rlang::sym\", \"{}\")", simple_identifier.name);
        //eval_string(&call).unwrap()
        r!(Symbol(&simple_identifier.name.clone()))
    },
    name: String,
);

define_ast_node!(
    BooleanLiteral,
    |_| Vec::new(),
    |lit: &BooleanLiteral, _py: Python, ast_module: &'a PyModule, _depth: usize| {
        ast_module.call1("Constant", (lit.val,))
    },
    |lit: &BooleanLiteral, _depth: usize| { r!(lit.val) },
    val: bool,
);
define_ast_node!(
    BigIntLiteral,
    |_| Vec::new(),
    |lit: &BigIntLiteral, _py: Python, ast_module: &'a PyModule, _depth: usize| {
        ast_module.call1("Constant", (lit.val,))
    },
    |lit: &BigIntLiteral, _depth: usize| { r!(lit.val) },
    // TODO: deprecate use of BigInt when removing rustpython
    val: i64,
);
define_ast_node!(
    PythonNone,
    |_| Vec::new(),
    |_, py: Python, ast_module: &'a PyModule, _depth: usize| {
        ast_module.call1("Constant", (py.None().as_ref(py),))
    },
    |_none, _depth| { r!(NULL) },
);

register_ast_nodes!(
    AST,
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
    Expression,
    Assignment,
    ForLoop,
    ImportNode,
    Add,
    BinOp,
);

impl Formatted {
    pub fn optimize(&self) -> Option<AST> {
        if self.keywords.len() == 1 {
            if let AST::StringLiteral(rw) = &self.fmt {
                let (unique_key, unique_value) = self.keywords.iter().next().unwrap().clone();
                let read = rw.read().unwrap();
                if read.value() == format!("{{{}}}", unique_key).to_string() {
                    return Some(unique_value.clone());
                }
            }
        }
        None
    }
}

impl AST {
    pub fn as_wrapped_assignment_target(&self) -> Self {
        match &self {
            AST::Subscript(ref x) => {
                AST::Subscript(x.read().unwrap().as_wrapped_assignment_target())
            }
            AST::Attribute(ref x) => {
                AST::Attribute(x.read().unwrap().as_wrapped_assignment_target())
            }
            AST::List(ref x) => AST::List(x.read().unwrap().as_wrapped_assignment_target()),
            AST::Tuple(ref x) => AST::Tuple(x.read().unwrap().as_wrapped_assignment_target()),
            AST::SimpleIdentifier(_) => self.clone(),
            _ => panic!("Assignment target not supported."),
        }
    }

    pub fn optimize(&self) -> Option<AST> {
        match self {
            AST::Formatted(ref rw) => {
                let read = rw.read().unwrap();
                read.optimize()
            }
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParameterTuple {
    args: Vec<AST>,
    pub kwargs: LinkedHashMap<String, AST>,
}
pub type ParameterTupleDedupKey = (usize, Vec<String>);
impl ParameterTuple {
    pub fn set_ancestors(&self, ancestors: Vec<AncestorRecord>) {
        for arg in self.args.iter() {
            arg.set_ancestors(ancestors.clone());
        }
        for v in self.kwargs.values() {
            v.set_ancestors(ancestors.clone());
        }
    }
    pub fn populate_python_dict(&self, dict: &mut LinkedHashMap<String, AST>) {
        if self.args.len() > 0 {
            dict.insert(
                "args".to_string(),
                AST::List(List::new_wrapped(self.args.clone(), false)),
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
        _object_uuid: Uuid,
        args_v: Vec<String>,
        kwargs_v: LinkedHashMap<String, String>,
        is_sql: bool,
    ) -> Self {
        let args = args_v
            .into_iter()
            .map(|x| AST::StringLiteral(StringLiteral::new_wrapped(x, is_sql)))
            .collect::<Vec<_>>();
        let kwargs = kwargs_v
            .into_iter()
            .map(|(k, v)| (k, AST::StringLiteral(StringLiteral::new_wrapped(v, is_sql))))
            .collect::<LinkedHashMap<_, _>>();
        Self { args, kwargs }
    }
    pub fn get_args(&self) -> Vec<AST> {
        self.args.clone()
    }
    pub fn get_kwargs(&self) -> LinkedHashMap<String, AST> {
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
            if let AST::StringLiteral(v) = arg {
                call = call.replace(&fmt, &v.read().unwrap().value());
            }
        }
        call
    }
}
#[allow(unused_imports)]
mod r_ast_tests {
    use crate::python::{Assignment, SimpleIdentifier, StringLiteral, AST};
    use extendr_api::prelude::*;
    #[test]
    fn test_string_literal() {
        test! {
            let s = StringLiteral::new_wrapped("test".to_string(), false);
            assert_eq!(s.read().unwrap().to_r_ast_node(0), r!("test"));
        }
    }
    #[test]
    fn test_simple_identifier() {
        test! {
            let s = SimpleIdentifier::new_wrapped("test".to_string());
            assert_eq!(s.read().unwrap().to_r_ast_node(0), sym!(test));
        }
    }
    #[test]
    fn test_assignment() {
        test! {
            let sym = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("a".to_string()));
            let val = AST::StringLiteral(StringLiteral::new_wrapped("b".to_string(), false));
            let assign = AST::Assignment(Assignment::new_wrapped(sym, val));
            let r_node = assign.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("call('<-', rlang::sym('a'), 'b')").unwrap());
        }
    }
}

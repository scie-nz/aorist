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
        r!(Lang(&[
            r!(Symbol("library")),
            import.inner.to_r_ast_node(depth)
        ]))
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

        r!(Lang(&[
            r!(Symbol("for")),
            for_loop.target.to_r_ast_node(depth),
            for_loop.iter.to_r_ast_node(depth),
            r!(Lang(&[r!(Symbol("{")), r!(List(pairlist)),]))
        ]))
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
        r!(Lang(&[
            r!(Symbol(op_str)),
            binop.left.to_r_ast_node(depth),
            binop.right.to_r_ast_node(depth)
        ]))
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
        let mut elems = list
            .elems
            .iter()
            .map(|x| x.to_r_ast_node(depth))
            .collect::<Vec<_>>();
        elems.insert(0, r!(Symbol("list")));
        r!(Lang(&elems))
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
        let elems = dict
            .elems
            .values()
            .map(|x| x.to_r_ast_node(depth))
            .collect::<Vec<_>>();
        let obj = r!(List(&elems));
        obj.set_names(dict.elems.keys()).unwrap();
        obj
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
        let mut args = LinkedHashMap::new();
        args.insert("n".to_string(), call.function.clone());
        for (k, v) in call.keywords.clone() {
            args.insert(k, v);
        }
        let mut elems = args
            .values()
            .map(|x| x.to_r_ast_node(depth))
            .collect::<Vec<_>>();
        elems.insert(0, r!("call"));
        let obj = r!(List(&elems));
        let mut names = args.keys().map(|x| x.clone()).collect::<Vec<_>>();
        names.insert(0, "name".to_string());
        obj.set_names(names).unwrap();
        call!("do.call", "call", obj, quote = TRUE).unwrap()
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
        let mut args = LinkedHashMap::new();
        for (k, v) in formatted.keywords.clone() {
            args.insert(k, v);
        }
        let mut elems = args
            .values()
            .map(|x| x.to_r_ast_node(depth))
            .collect::<Vec<_>>();
        elems.insert(0, formatted.fmt.to_r_ast_node(depth));
        elems.insert(0, r!("glue::glue"));
        elems.insert(0, r!("call"));
        let obj = r!(List(&elems));
        let mut names = args.keys().map(|x| x.clone()).collect::<Vec<_>>();
        names.insert(0, "fmt".to_string());
        names.insert(0, "name".to_string());
        names.insert(0, "name".to_string());
        obj.set_names(names).unwrap();
        call!("do.call", "call", obj, quote = TRUE).unwrap()
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
        r!(Lang(&[
            r!(Symbol("[[")),
            a_node,
            b_node,
        ]))
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
    |lit: &BooleanLiteral, _depth: usize| { Robj::from(lit.val) },
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
    None,
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
    None,
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
    use crate::python::*;
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
    #[test]
    fn test_import() {
        test! {
            let sym = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("ggplot".to_string()));
            let import = AST::ImportNode(ImportNode::new_wrapped(sym));
            let r_node = import.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("call('library', rlang::sym('ggplot'))").unwrap());
        }
    }
    #[test]
    fn test_for_loop() {
        test! {
            let it = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("i".to_string()));
            let vec = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("vec".to_string()));

            let sym = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("a".to_string()));
            let assign = AST::Assignment(Assignment::new_wrapped(sym, it.clone()));
            let for_loop = AST::ForLoop(ForLoop::new_wrapped(it, vec, vec![assign]));
            let r_node = for_loop.to_r_ast_node(0);
            assert_eq!(r_node, eval_string(
                "call('for', rlang::sym('i'), rlang::sym('vec'), call('{', list(call('<-', rlang::sym('a'), rlang::sym('i')))))"
            ).unwrap());
        }
    }
    #[test]
    fn test_expression() {
        test! {
            let sym = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("ggplot".to_string()));
            let expr = AST::Expression(Expression::new_wrapped(sym));
            assert_eq!(expr.to_r_ast_node(0), sym!(ggplot));
        }
    }
    #[test]
    fn test_binop() {
        test! {
            let sym_a = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("a".to_string()));
            let sym_b = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("b".to_string()));
            let op = AST::Add(Add::new_wrapped());
            let binop = AST::BinOp(BinOp::new_wrapped(sym_a, op, sym_b));
            let r_node = binop.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("call('+', rlang::sym('a'), rlang::sym('b'))").unwrap());
        }
    }
    #[test]
    fn test_list() {
        test! {
            let sym_a = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("a".to_string()));
            let sym_b = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("b".to_string()));
            let list = AST::List(crate::python::List::new_wrapped(vec![sym_a, sym_b], false));
            let r_node = list.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("call('list', rlang::sym('a'), rlang::sym('b'))").unwrap());
        }
    }
    #[test]
    fn test_dict() {
        test! {
            let sym_a = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("a".to_string()));
            let sym_b = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("b".to_string()));
            let mut map = linked_hash_map::LinkedHashMap::new();
            map.insert("x".to_string(), sym_a);
            map.insert("y".to_string(), sym_b);
            let dict = AST::Dict(crate::python::Dict::new_wrapped(map));
            let r_node = dict.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("list(x=rlang::sym('a'), y=rlang::sym('b'))").unwrap());
            // N.B.: this also evaluates as correct -- names don't seem to matter
            assert_eq!(r_node, eval_string("list(z=rlang::sym('a'), y=rlang::sym('b'))").unwrap());
        }
    }
    #[test]
    fn test_call() {
        test! {
            let sym_fun = AST::StringLiteral(StringLiteral::new_wrapped("fun".to_string(), false));
            let sym_a = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("a".to_string()));
            let sym_b = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("b".to_string()));
            let mut map = linked_hash_map::LinkedHashMap::new();
            map.insert("x".to_string(), sym_a);
            map.insert("y".to_string(), sym_b);
            let dict = AST::Call(crate::python::Call::new_wrapped(sym_fun, vec![], map));
            let r_node = dict.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("call('call', name='fun', x=rlang::sym('a'), y=rlang::sym('b'))").unwrap());
        }
    }
    #[test]
    fn test_fmt() {
        test! {
            let fmt = AST::StringLiteral(StringLiteral::new_wrapped("{x} {y}".to_string(), false));
            let sym_a = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("a".to_string()));
            let sym_b = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("b".to_string()));
            let mut map = linked_hash_map::LinkedHashMap::new();
            map.insert("x".to_string(), sym_a);
            map.insert("y".to_string(), sym_b);
            let dict = AST::Formatted(crate::python::Formatted::new_wrapped(fmt, map));
            let r_node = dict.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("call('call', name='glue::glue', fmt='{x} {y}', x=rlang::sym('a'), y=rlang::sym('b'))").unwrap());
        }
    }
    #[test]
    fn test_subscript() {
        test! {
            let sym_a = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("a".to_string()));
            let sym_b = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("b".to_string()));
            let subscript = AST::Subscript(crate::python::Subscript::new_wrapped(sym_a, sym_b, false));
            let r_node = subscript.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("quote(a[[b]])").unwrap());
            
        }
    }
    
    #[test]
    fn test_boolean_literal() {
        test! {
            let sym = AST::BooleanLiteral(BooleanLiteral::new_wrapped(true));
            let r_node = sym.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("quote(TRUE)").unwrap());
        }
    }
    
    #[test]
    fn test_bigint_literal() {
        test! {
            let sym = AST::BigIntLiteral(BigIntLiteral::new_wrapped(1));
            let r_node = sym.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("as.integer(1)").unwrap());
        }
    }
    
    #[test]
    fn test_none() {
        test! {
            let sym = AST::None(None::new_wrapped());
            let r_node = sym.to_r_ast_node(0);
            assert_eq!(r_node, eval_string("quote(NULL)").unwrap());
        }
    }
}

mod ancestor_record;
mod assignment_target;
mod extract_arg;
mod string_literal;

pub use ancestor_record::*;
pub use assignment_target::*;
pub use extract_arg::*;
pub use string_literal::*;

use aorist_derive::Optimizable;
use aorist_extendr_api::prelude::*;
use aorist_primitives::{define_ast_node, register_ast_nodes};
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyString, PyTuple};
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use abi_stable::std_types::RArc;
use std::sync::RwLock;

define_ast_node!(
    ImportNode,
    |import: &ImportNode| vec![import.inner.clone()],
    |import: &ImportNode, py: Python, ast_module: &'a PyModule, depth: usize| {
        import.to_python_ast_node(py, ast_module, depth)
    },
    |import: &ImportNode, depth: usize| {
        r!(Language::from_values(&[
            r!(Symbol::from_string("library")),
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
            .map(|x| x.to_python_ast_node(py, ast_module, depth + 1))
            .collect::<PyResult<Vec<_>>>()?;
        let body_list = PyList::new(py, body_ast);
        let empty_vec: Vec<String> = Vec::new();
        let empty_list = PyList::new(py, empty_vec);
        ast_module.getattr("For")?.call1((
            for_loop.target.to_python_ast_node(py, ast_module, depth)?,
            for_loop.iter.to_python_ast_node(py, ast_module, depth)?,
            body_list.as_ref(),
            empty_list.as_ref(),
        ))
    },
    |for_loop: &ForLoop, depth: usize| {
        unsafe {
            let res = make_lang("call");
            let mut tail = res.get();
            tail = append_with_name(tail, r!("for"), "name");
            tail = append(tail, for_loop.target.to_r_ast_node(depth));
            tail = append(tail, for_loop.iter.to_r_ast_node(depth));

            let body = make_lang("call");
            let mut body_tail = body.get();
            body_tail = append_with_name(body_tail, r!("{"), "name");
            for x in for_loop.body.iter() {
                body_tail = append(body_tail, x.to_r_ast_node(depth));
            }
            let _ = body_tail;

            tail = append(tail, body);

            let _ = tail;
            res
        }
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
        ast_module.getattr("Assign")?.call1((
            targets,
            assign.call.to_python_ast_node(py, ast_module, depth)?,
        ))
    },
    |assign: &Assignment, depth: usize| {
        unsafe {
            let res = make_lang("call");
            let mut tail = res.get();
            tail = append_with_name(tail, r!("<-"), "name");
            tail = append(tail, assign.target.to_r_ast_node(depth));
            tail = append(tail, assign.call.to_r_ast_node(depth));
            let _ = tail;
            res
        }
    },
    target: AST,
    call: AST,
);
define_ast_node!(
    Expression,
    |expr: &Expression| vec![expr.inner.clone()],
    |expr: &Expression, py: Python, ast_module: &'a PyModule, depth: usize| {
        ast_module
            .getattr("Expr")?
            .call1((expr.inner.to_python_ast_node(py, ast_module, depth)?,))
    },
    |expr: &Expression, depth: usize| { expr.inner.to_r_ast_node(depth) },
    inner: AST,
);
define_ast_node!(
    Add,
    |_node: &Add| vec![],
    |_node: &Add, _py: Python, ast_module: &'a PyModule, _depth: usize| {
        ast_module.getattr("Add")?.call0()
    },
    |_add: &Add, _depth: usize| { panic!("Should not call to_r_ast_node on Add objects directly") },
);
define_ast_node!(
    BinOp,
    |node: &BinOp| vec![node.left.clone(), node.right.clone()],
    |node: &BinOp, py: Python, ast_module: &'a PyModule, depth: usize| {
        ast_module.getattr("BinOp")?.call1((
            node.left.to_python_ast_node(py, ast_module, depth)?,
            node.op.to_python_ast_node(py, ast_module, depth)?,
            node.right.to_python_ast_node(py, ast_module, depth)?,
        ))
    },
    |binop: &BinOp, depth: usize| {
        let op_str = match binop.op {
            AST::Add(_) => "+",
            _ => panic!("AST node not supported as R operator"),
        };
        r!(Language::from_values(&[
            r!(Symbol::from_string(op_str)),
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
            .getattr(match list.store {
                true => "Store",
                false => "Load",
            })?
            .call0()?;
        let children = list
            .elems
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module, depth))
            .collect::<PyResult<Vec<_>>>()?;
        let children_list = PyList::new(py, children);
        ast_module
            .getattr("List")?
            .call1((children_list.as_ref(), mode))
    },
    |list: &List, depth: usize| {
        let mut elems = list
            .elems
            .iter()
            .map(|x| x.to_r_ast_node(depth))
            .collect::<Vec<_>>();
        elems.insert(0, r!(Symbol::from_string("list")));
        r!(Language::from_values(&elems))
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
                StringLiteral::new(x.clone(), false).to_python_ast_node(py, ast_module, depth + 1)
            })
            .collect::<PyResult<Vec<_>>>()?;
        let values = dict
            .elems
            .values()
            .map(|x| x.to_python_ast_node(py, ast_module, depth + 1))
            .collect::<PyResult<Vec<_>>>()?;
        let keys_list = PyList::new(py, keys);
        let values_list = PyList::new(py, values);
        ast_module
            .getattr("Dict")?
            .call1((keys_list.as_ref(), values_list.as_ref()))
    },
    |dict: &Dict, depth: usize| {
        let elems = dict
            .elems
            .values()
            .map(|x| x.to_r_ast_node(depth))
            .collect::<Vec<_>>();
        let obj = r!(aorist_extendr_api::List::from_values(&elems));
        obj.set_names(dict.elems.keys()).unwrap();
        obj
    },
    elems: LinkedHashMap<String, AST>,
);
impl Dict {
    pub fn replace_elem(&mut self, key: String, elem: AST) {
        self.elems.insert(key, elem);
    }
    pub fn len(&self) -> usize {
        self.elems.len()
    }
}
define_ast_node!(
    Tuple,
    |tuple: &Tuple| tuple.elems().iter().cloned().collect::<Vec<AST>>(),
    |tuple: &Tuple, py: Python, ast_module: &'a PyModule, depth: usize| {
        let mode = ast_module
            .getattr(match tuple.store {
                true => "Store",
                false => "Load",
            })?
            .call0()?;
        let children = tuple
            .elems
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module, depth + 1))
            .collect::<PyResult<Vec<_>>>()?;
        let children_list = PyList::new(py, children);
        ast_module
            .getattr("Tuple")?
            .call1((children_list.as_ref(), mode))
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
            .getattr(match attribute.store {
                true => "Store",
                false => "Load",
            })?
            .call0()?;
        let val_ast = attribute.value.to_python_ast_node(py, ast_module, depth)?;
        let name_ast = PyString::new(py, &attribute.name);
        ast_module
            .getattr("Attribute")?
            .call1((val_ast, name_ast.as_ref(), mode))
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
            .map(|x| x.to_python_ast_node(py, ast_module, depth + 1))
            .collect::<PyResult<Vec<_>>>()?;
        let kwargs = call
            .keywords
            .iter()
            .map(|(k, v)| {
                ast_module.getattr("keyword")?.call1(PyTuple::new(
                    py,
                    &vec![
                        PyString::new(py, k).as_ref(),
                        v.to_python_ast_node(py, ast_module, depth + 1)?,
                    ],
                ))
            })
            .collect::<PyResult<Vec<_>>>()?;
        let function = call.function.to_python_ast_node(py, ast_module, depth)?;
        ast_module.getattr("Call")?.call1((function, args, kwargs))
    },
    |call: &Call, depth: usize| {
        unsafe {
            let fn_name = match call.function {
                AST::SimpleIdentifier(ref x) => x.read().unwrap().name(),
                _ => panic!("function name must be SimpleIdentifier"),
            };
            let res = make_lang("call");
            let mut tail = res.get();
            tail = append_with_name(tail, r!(fn_name), "name");
            for arg in &call.args {
                tail = append(tail, arg.to_r_ast_node(depth));
            }
            for (k, v) in &call.keywords {
                tail = append_with_name(tail, v.to_r_ast_node(depth), k);
            }
            let _ = tail;
            res
        }
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
        let format_fn = ast_module.getattr("Attribute")?.call1((
            formatted.fmt.to_python_ast_node(py, ast_module, depth)?,
            PyString::new(py, "format").as_ref(),
        ))?;
        let kwargs = formatted
            .keywords
            .iter()
            .map(|(k, v)| {
                ast_module.getattr("keyword")?.call1(PyTuple::new(
                    py,
                    &vec![
                        PyString::new(py, k).as_ref(),
                        v.to_python_ast_node(py, ast_module, depth + 1)?,
                    ],
                ))
            })
            .collect::<PyResult<Vec<_>>>()?;
        let args: Vec<String> = Vec::new();
        ast_module.getattr("Call")?.call1((
            format_fn,
            PyList::new(py, args).as_ref(),
            PyList::new(py, kwargs).as_ref(),
        ))
    },
    |formatted: &Formatted, depth: usize| {
        let mut args = LinkedHashMap::new();
        for (k, v) in formatted.keywords.clone() {
            args.insert(k, v);
        }
        unsafe {
            let res = make_lang("call");
            let mut tail = res.get();
            tail = append_with_name(tail, r!("glue"), "name");
            tail = append(tail, formatted.fmt.to_r_ast_node(depth));
            for (k, v) in &formatted.keywords {
                tail = append_with_name(tail, v.to_r_ast_node(depth), k);
            }
            let _ = tail;
            res
        }
    },
    fmt: AST,
    keywords: LinkedHashMap<String, AST>,
);
define_ast_node!(
    Subscript,
    |subscript: &Subscript| vec![subscript.a().clone(), subscript.b().clone()],
    |subscript: &Subscript, py: Python, ast_module: &'a PyModule, depth: usize| {
        let mode = ast_module
            .getattr(match subscript.store {
                true => "Store",
                false => "Load",
            })?
            .call0()?;
        let b_node = subscript.b.to_python_ast_node(py, ast_module, depth + 1)?;
        let idx = ast_module.getattr("Index")?.call1((b_node,))?;
        let value = subscript.a.to_python_ast_node(py, ast_module, depth + 1)?;
        ast_module.getattr("Subscript")?.call1((value, idx, mode))
    },
    |subscript: &Subscript, depth: usize| {
        let a_node = subscript.a.to_r_ast_node(depth);
        let b_node = subscript.b.to_r_ast_node(depth);
        unsafe {
            let res = make_lang("call");
            let mut tail = res.get();
            tail = append_with_name(tail, r!("[["), "name");
            tail = append(tail, a_node);
            tail = append(tail, b_node);
            let _ = tail;
            res
        }
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
        ast_module
            .getattr("Name")?
            .call1((PyString::new(py, &simple_identifier.name).as_ref(),))
    },
    |simple_identifier: &SimpleIdentifier, _depth: usize| {
        call!("call", r!("as.name"), r!(&simple_identifier.name)).unwrap()
    },
    name: String,
);

define_ast_node!(
    BooleanLiteral,
    |_| Vec::new(),
    |lit: &BooleanLiteral, _py: Python, ast_module: &'a PyModule, _depth: usize| {
        ast_module.getattr("Constant")?.call1((lit.val,))
    },
    |lit: &BooleanLiteral, _depth: usize| { Robj::from(lit.val) },
    val: bool,
);

define_ast_node!(
    If,
    |if_else: &If| vec![if_else.test.clone()]
        .into_iter()
        .chain(if_else.body.clone().into_iter())
        .chain(match &if_else.orelse {
            Some(x) => x.clone().into_iter(),
            None => vec![].into_iter(),
        })
        .collect(),
    |if_else: &If, py: Python, ast_module: &'a PyModule, depth: usize| {
        let body_ast = if_else
            .body
            .iter()
            .map(|x| match &x {
                AST::Assignment(_) | AST::Expression(_) => x,
                _ => panic!("AST node of type {} found in if body", x.name()),
            })
            .map(|x| x.to_python_ast_node(py, ast_module, depth + 1))
            .collect::<PyResult<Vec<_>>>()?;
        let orelse_ast = match &if_else.orelse {
            Some(x) => x
                .iter()
                .map(|x| match &x {
                    AST::Assignment(_) | AST::Expression(_) => x,
                    _ => panic!("AST node of type {} found in if orelse", x.name()),
                })
                .map(|x| x.to_python_ast_node(py, ast_module, depth + 1))
                .collect::<PyResult<Vec<_>>>()?,
            None => Vec::new(),
        };

        let body_list = PyList::new(py, body_ast);
        let orelse_list = PyList::new(py, orelse_ast);

        ast_module.getattr("If")?.call1((
            if_else.test.to_python_ast_node(py, ast_module, depth)?,
            body_list.as_ref(),
            orelse_list.as_ref(),
        ))
    },
    |_if_else: &If, _depth: usize| {
        panic!("if - else unimplemented in R");
    },
    test: AST,
    body: Vec<AST>,
    orelse: Option<Vec<AST>>,
);
define_ast_node!(
    BigIntLiteral,
    |_| Vec::new(),
    |lit: &BigIntLiteral, _py: Python, ast_module: &'a PyModule, _depth: usize| {
        ast_module.getattr("Constant")?.call1((lit.val,))
    },
    |lit: &BigIntLiteral, _depth: usize| { r!(lit.val) },
    // TODO: deprecate use of BigInt when removing rustpython
    val: i64,
);
define_ast_node!(
    FloatLiteral,
    |_| Vec::new(),
    |lit: &FloatLiteral, _py: Python, ast_module: &'a PyModule, _depth: usize| {
        ast_module.getattr("Constant")?.call1((lit.val.as_f64(),))
    },
    |lit: &FloatLiteral, _depth: usize| { r!(lit.val.as_f64()) },
    val: aorist_attributes::FloatValue,
);
define_ast_node!(
    None,
    |_| Vec::new(),
    |_, py: Python, ast_module: &'a PyModule, _depth: usize| {
        ast_module
            .getattr("Constant")?
            .call1((py.None().as_ref(py),))
    },
    |_none, _depth| { r!(NULL) },
);
define_ast_node!(
    FunctionDef,
    |_fun: &FunctionDef| vec![],
    |fun: &FunctionDef, py: Python, ast_module: &'a PyModule, depth: usize| {
        let mut args_py = Vec::new();
        let mut kwargs_py = Vec::new();
        let mut kwargs_defaults = Vec::new();
        for arg in &fun.args {
            let arg_py = ast_module.getattr("arg")?.call1((
                arg.to_python_ast_node(py, ast_module, depth)?,
                py.None().as_ref(py),
            ))?;
            args_py.push(arg_py);
        }
        for (k, v) in &fun.kwargs {
            let arg_py = ast_module.getattr("arg")?.call1((
                v.to_python_ast_node(py, ast_module, depth)?,
                py.None().as_ref(py),
            ))?;
            kwargs_py.push(arg_py);
            let default_py = AST::StringLiteral(StringLiteral::new_wrapped(k.clone(), false))
                .to_python_ast_node(py, ast_module, depth)?;
            kwargs_defaults.push(default_py);
        }
        let arguments = ast_module.getattr("arguments")?.call1((
            args_py,
            py.None().as_ref(py),
            kwargs_py,
            py.None().as_ref(py),
            py.None().as_ref(py),
            kwargs_defaults,
        ))?;
        let body_py = fun
            .body
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module, depth))
            .collect::<PyResult<Vec<_>>>()?;
        ast_module
            .getattr("FunctionDef")?
            .call1((&fun.name, arguments, body_py))
    },
    |_fun: &FunctionDef, _depth: usize| { panic!("Function defs not supported in R") },
    name: String,
    args: Vec<AST>,
    kwargs: LinkedHashMap<String, AST>,
    body: Vec<AST>,
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
    FunctionDef,
    FloatLiteral,
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

    pub fn to_python_source(&self) -> PyResult<String> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let ast = PyModule::import(py, "ast").unwrap();
        let node = self.to_python_ast_node(py, ast, 0)?;
        let module = ast.getattr("Expression")?.call1((node,))?;
        let astor = PyModule::import(py, "astor").unwrap();
        let source = astor.getattr("to_source")?.call1((module,));
        source.and_then(|x| Ok(x.to_string()))
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

#[allow(unused_imports)]
mod r_ast_tests {
    use crate::*;
    use aorist_extendr_api::prelude::*;

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
            let expr = AST::Expression(crate::python::Expression::new_wrapped(sym));
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
            assert_eq!(r_node, eval_string("call('call', name='glue', fmt='{x} {y}', x=rlang::sym('a'), y=rlang::sym('b'))").unwrap());
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

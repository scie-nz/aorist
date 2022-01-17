#![allow(non_snake_case)]

mod concept;
pub use concept::*;
mod endpoints;
pub use endpoints::*;
mod context;
pub use context::*;
mod dialect;
pub use dialect::{Dialect, Bash, R, Presto, Python};
#[cfg(feature = "python")]
pub use dialect::dialects_module;
mod program;
pub use program::*;

#[macro_export]
macro_rules! register_ast_nodes {
    ($name:ident, $($variant: ident,)+) => {

        #[derive(Clone)]
        pub enum $name {
            $(
                $variant(RArc<RRwLock<$variant>>),
            )+
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                match (&self) {
                    $(
                        Self::$variant(v) => v.read().fmt(f),
                    )+
                }
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                match (&self, other) {
                    $(
                        (Self::$variant(v1), Self::$variant(v2)) => {
                            v1.read().eq(&v2.read())
                        },
                    )+
                    (_, _) => false,
                }
            }
        }
        impl Eq for $name {}
        impl $name {
            pub fn clone_without_ancestors(&self) -> Self {
                match &self {
                    $(
                        Self::$variant(x) => Self::$variant(RArc::new(RRwLock::new(x.read().clone_without_ancestors()))),
                    )+
                }
            }
            pub fn set_ancestors(&self, ancestors: AVec<AncestorRecord>) {
                match &self {
                    $(
                        Self::$variant(x) => x.write().set_ancestors(ancestors),
                    )+
                }
            }
            pub fn get_ancestors(&self) -> AOption<AVec<AncestorRecord>> {
                match &self {
                    $(
                        Self::$variant(x) => x.read().get_ancestors(),
                    )+
                }
            }
            pub fn get_descendants(&self) -> AVec<AST> {
                let mut queue = VecDeque::new();
                queue.push_back(self.clone());
                let mut current = queue.pop_front();
                let mut v: AVec<AST> = AVec::new();
                while let Some(elem) = current {
                    let direct_descendants = match &elem {
                        $(
                            Self::$variant(x) => {
                            let read = x.read();
                            read.get_direct_descendants()
                            }
                        )+
                    };
                    for desc in direct_descendants.into_iter() {
                        queue.push_back(desc);
                    }
                    v.push(elem);
                    current = queue.pop_front();
                }
                v
            }
            pub fn name(&self) -> String {
                match &self {
                    $(
                        Self::$variant(..) => stringify!($variant),
                    )+
                }
                .to_string()
            }
            pub fn optimize_fields(&self) {
                match &self {
                    $(
                        Self::$variant(rw) => rw.write().optimize_fields(),
                    )+
                }
            }
            pub fn to_python_ast_node<'a>(
                &self,
                py: Python,
                ast_module: &'a PyModule,
                depth: usize,
            ) -> PyResult<&'a PyAny> {
                match &self {
                    $(
                        Self::$variant(x) => x.read().to_python_ast_node(
                            py,
                            ast_module,
                            depth,
                        ),
                    )+
                }
            }
            pub fn to_r_ast_node(
                &self,
                depth: usize,
            ) -> Robj {
                match &self {
                    $(
                        Self::$variant(x) => x.read().to_r_ast_node(
                            depth,
                        ),
                    )+
                }
            }
        }
        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                match &self {
                    $(
                        Self::$variant(x) => x.read().hash(state),
                    )+
                }
            }
        }
    }
}

#[macro_export]
macro_rules! define_task_node {
    ($name:ident,
     $descendants:expr,
     $py_ast_closure:expr,
     $import_closure:expr,
     $import_type:ty,
     $($field: ident : $field_type: ty,)*) => {
        #[derive(Hash, PartialEq, Clone)]
        pub struct $name {
            $(
                $field: $field_type,
            )*
        }
        impl $name {
            pub fn new_wrapped($(
                $field: $field_type,
            )*) -> RArc<RRwLock<Self>> {
                RArc::new(RRwLock::new(Self::new($($field, )*)))
            }
            pub fn get_statements<'a>(
                &self,
            ) -> AVec<AST> {
                ($py_ast_closure)(self)
            }
            pub fn new($(
                $field: $field_type,
            )*) -> Self {
                Self {
                    $($field,)*
                }
            }
            $(
                pub fn $field(&self) -> $field_type {
                    self.$field.clone()
                }
            )*
            pub fn get_direct_descendants(&self) -> AVec<AST> {
                $descendants(self)
            }
            pub fn get_imports(&self) -> AVec<$import_type> {
                $import_closure(self)
            }
        }
    };
}

#[macro_export]
macro_rules! register_task_nodes {
    ($name:ident, $import_type: ty, $($variant: ident,)+) => {

        #[derive(Clone)]
        pub enum $name {
            $(
                $variant(RArc<RRwLock<$variant>>),
            )+
        }
        impl $name {
            pub fn get_imports(&self) -> AVec<$import_type>{
                match &self {
                    $(
                        Self::$variant(x) => x.read().get_imports(),
                    )+
                }
            }
            pub fn get_statements(&self) -> AVec<AST> {
                match &self {
                    $(
                        Self::$variant(x) => x.read().get_statements(),
                    )+
                }
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                match (&self, other) {
                    $(
                        (Self::$variant(v1), Self::$variant(v2)) => {
                            v1.read().eq(&v2.read())
                        },
                    )+
                    (_, _) => false,
                }
            }
        }
        impl Eq for $name {}
        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                match &self {
                    $(
                        Self::$variant(x) => x.read().hash(state),
                    )+
                }
            }
        }
    }
}

#[macro_export]
macro_rules! define_ast_node {
    ($name:ident,
     $descendants:expr,
     $py_ast_closure:expr,
     $r_ast_closure:expr,
     $($field: ident : $field_type: ty,)*) => {
        #[derive(Hash, PartialEq, Eq, Clone, Optimizable, Debug)]
        pub struct $name {
            $(
                $field: $field_type,
            )*
            ancestors: AOption<AVec<AncestorRecord>>,
        }
        impl $name {
            pub fn new_wrapped($(
                $field: $field_type,
            )*) -> RArc<RRwLock<Self>> {
                RArc::new(RRwLock::new(Self::new($($field, )*)))
            }
            pub fn to_python_ast_node<'a>(
                &self,
                py: Python,
                ast_module: &'a PyModule,
                depth: usize,
            ) -> PyResult<&'a PyAny> {
                ($py_ast_closure)(self, py, ast_module, depth)
            }
            pub fn to_r_ast_node(&self, depth: usize) -> Robj {
                ($r_ast_closure)(self, depth)
            }
            pub fn new($(
                $field: $field_type,
            )*) -> Self {
                Self {
                    $($field,)*
                    ancestors: AOption(ROption::RNone),
                }
            }
            pub fn clone_without_ancestors(&self) -> Self {
                Self {
                    $($field: self.$field.clone(),)*
                    ancestors: AOption(ROption::RNone),
                }
            }
            pub fn set_ancestors(&mut self, ancestors: AVec<AncestorRecord>) {
                assert!(self.ancestors.is_none());
                self.ancestors = AOption(ROption::RSome(ancestors));
            }
            pub fn get_ancestors(&self) -> AOption<AVec<AncestorRecord>> {
                self.ancestors.clone()
            }
            $(
                pub fn $field(&self) -> $field_type {
                    self.$field.clone()
                }
            )*
            pub fn get_direct_descendants(&self) -> AVec<AST> {
                $descendants(self)
            }
        }
    };
}

#[macro_export]
macro_rules! define_program {
    ($name:ident, $root:ident, $constraint:ident, $satisfy_type:ident,
     $lt: lifetime, $clt: lifetime,
     $dialect:ident,
     $preamble:expr, $call:expr, $tuple_call: expr, $dialect_call: expr) => {
        pub struct $name {}
        impl<$lt, $clt> ConstraintSatisfactionBase<$lt, $clt> for $name
        where
            $lt: $clt,
        {
            type RootType = $root;
            type ConstraintType = $constraint;
            type Outer = Constraint;
        }
        impl<$lt, $clt> $satisfy_type<$lt, $clt> for $name
        where
            $lt: $clt,
        {
            type Dialect = $dialect;
            fn compute_parameter_tuple(
                uuid: AUuid,
                c: Concept<'a>,
                ancestry: RArc<ConceptAncestry<'a>>,
            ) -> ParameterTuple {
                $tuple_call(uuid, c, ancestry)
            }
            fn get_preamble() -> String {
                $preamble.to_string()
            }
            fn get_call() -> String {
                $call.to_string()
            }
            fn get_dialect() -> Dialect {
                Dialect::$dialect($dialect_call())
            }
        }
    };
}

#[macro_export]
macro_rules! define_attribute {
    (
      $element:ident,
      $presto_type:ident,
      $orc_type:ident,
      $sql_type:ident,
      $sqlite_type:ident,
      $postgres_type:ident,
      $bigquery_type:ident,
      $value:ident,
      $key:expr,
      $pyo3_type: ty
    ) => {
        aorist_paste::item! {
            #[repr(C)]
            #[cfg_attr(feature = "python", pyclass(module = "aorist"))]
            #[derive(
                Hash,
                PartialEq,
                Eq,
                Debug,
                Serialize,
                Deserialize,
                Clone,
                $presto_type,
                $orc_type,
                $sqlite_type,
                $postgres_type,
                $bigquery_type,
                abi_stable::StableAbi
            )]
            #[cfg_attr(feature = "sql", derive($sql_type))]
            pub struct $element {
                pub name: AString,
                pub comment: AOption<AString>,
                pub nullable: bool,
            }
            impl TAttribute for $element {
                type Value = $value;

                fn get_name(&self) -> AString {
                    self.name.clone()
                }
                fn get_comment(&self) -> AOption<AString> {
                    self.comment.clone()
                }
                fn is_nullable(&self) -> bool {
                    self.nullable
                }
                fn is_key_type() -> bool {
                    $key
                }
            }
            #[cfg(feature = "python")]
            impl $element {
                pub fn get_py_type(&self) -> PyResult<pyo3::prelude::PyObject> {
                    let gil_guard = pyo3::prelude::Python::acquire_gil();
                    let py = gil_guard.python();
                    Ok(pyo3::types::PyType::new::<$pyo3_type>(py).to_object(py))
                }
            }
            #[cfg(feature = "python")]
            #[pymethods]
            impl $element {
                #[new]
                #[args(comment = "None")]
                #[args(nullable = "false")]
                pub fn new(
                    name: &str,
                    comment: Option<&str>,
                    nullable: bool
                ) -> Self {
                    let comment_str: ROption<AString> = match comment {
                        Some(x) => ROption::RSome(x.into()),
                        None => ROption::RNone,
                    };
                    Self { name: name.into(), comment: AOption(comment_str), nullable }
                }
                #[getter]
                pub fn name(&self) -> PyResult<&str> {
                    Ok(self.name.as_str())
                }
                #[getter]
                pub fn py_type(&self) -> PyResult<pyo3::prelude::PyObject> {
                    self.get_py_type()
                }
            }
            #[cfg(feature = "python")]
            #[pyo3::prelude::pyproto]
            impl pyo3::PyObjectProtocol for $element {
                fn __repr__(&self) -> pyo3::PyResult<String> {
                    Ok(format!(
                        "{} {}",
                        stringify!($element),
                        serde_json::to_string_pretty(self).unwrap()
                    ))
                }
                fn __str__(&self) -> pyo3::PyResult<String> {
                    Ok(format!(
                        "{} {}",
                        stringify!($element),
                        serde_json::to_string_pretty(self).unwrap()
                    ))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_constraint {
    ($element:ident, $requires_program:expr, $satisfy_type:ident, $root:ident, $outer:ident,
    $title:expr, $body:expr, $should_add:expr, $get_required:expr $(, $required:ident)*) => {
        aorist_paste::item! {

            #[repr(C)]
            #[cfg_attr(feature = "python", pyclass(module = "aorist"))]
            #[derive(Clone)]
            pub struct $element {
                id: AUuid,
                root_uuid: AUuid,
                $([<$required:snake:lower>] : Vec<RArc<RRwLock<$outer>>>,)*
            }
            #[cfg(feature = "python")]
            #[pymethods]
            impl $element {
                #[classattr]
                pub fn name() -> String {
                    stringify!($element).to_string()
                }
                #[classattr]
                pub fn required() -> Vec<String> {
                    vec![
                        $(
                            stringify!($required).into(),
                        )*
                    ]
                }
                #[classattr]
                pub fn root() -> String {
                    stringify!($root).into()
                }
                #[classattr]
                pub fn program_required() -> bool {
                    $requires_program
                }
            }
            pub trait $satisfy_type<'a> : ConstraintSatisfactionBase<'a, ConstraintType=$element, RootType=$root> {
                type Dialect;

                // computes a parameter tuple as a string, e.g. to be called from
                // Python
                fn compute_parameter_tuple(
                    uuid: AUuid,
                    root: Concept,
                    ancestry: RArc<ConceptAncestry>,
                ) -> ParameterTuple;
                fn get_preamble() -> String;
                fn get_call() -> String;
                fn get_dialect() -> Dialect;
            }

            #[repr(C)]
            #[cfg_attr(feature = "python", pyclass(module = "aorist"))]
            #[derive(Clone)]
            pub struct [<$element Program>] {
                dialect: Dialect,
                code: AString,
                entrypoint: AString,
                arg_functions: Vec<(Vec<AString>, AString)>,
                kwarg_functions: LinkedHashMap<AString, (Vec<AString>, AString)>,
            }
            impl [<$element Program>] {
                pub fn new(
                    code: &str,
                    entrypoint: &str,
                    arg_functions: Vec<(Vec<&str>, &str)>,
                    kwarg_functions: HashMap<&str, (Vec<&str>, &str)>,
                    dialect: Dialect,
                ) -> Self {
                    let mut funs: LinkedHashMap<AString, (Vec<AString>, AString)> = LinkedHashMap::new();
                    for (k, (v1, v2)) in kwarg_functions.into_iter() {
                        funs.insert(k.into(), (v1.into_iter().map(|x| x.into()).collect(), v2.into()));
                    }
                    Self {
                        code: code.into(),
                        entrypoint: entrypoint.into(),
                        arg_functions: arg_functions.into_iter().map(|(x, y)| (x.into_iter().map(|x| x.into()).collect(), y.into())).collect(),
                        kwarg_functions: funs,
                        dialect: dialect,
                    }
                }
            }

            #[cfg(feature = "python")]
            #[pymethods]
            impl $element {
                #[staticmethod]
                pub fn register_python_program(
                    code: &str,
                    entrypoint: &str,
                    arg_functions: Vec<(Vec<&str>, &str)>,
                    kwarg_functions: HashMap<&str, (Vec<&str>, &str)>,
                    pip_requirements: Vec<&str>,
                ) -> PyResult<[<$element Program>]> {

                    Ok([<$element Program>]::new(
                        code, 
                        entrypoint, 
                        arg_functions, 
                        kwarg_functions, 
                        Dialect::Python(aorist_core::Python::new(pip_requirements))
                    ))
                }
                #[staticmethod]
                pub fn register_r_program(
                    code: &str,
                    entrypoint: &str,
                    arg_functions: Vec<(Vec<&str>, &str)>,
                    kwarg_functions: HashMap<&str, (Vec<&str>, &str)>,
                ) -> PyResult<[<$element Program>]> {
                    Ok([<$element Program>]::new(
                        code, 
                        entrypoint, 
                        arg_functions, 
                        kwarg_functions, 
                        Dialect::R(aorist_core::R::new()),
                    ))
                }
                #[staticmethod]
                pub fn register_presto_program(
                    code: &str,
                    entrypoint: &str,
                    arg_functions: Vec<(Vec<&str>, &str)>,
                    kwarg_functions: HashMap<&str, (Vec<&str>, &str)>,
                ) -> PyResult<[<$element Program>]> {
                    Ok([<$element Program>]::new(
                        code, 
                        entrypoint, 
                        arg_functions, 
                        kwarg_functions, 
                        Dialect::Presto(aorist_core::Presto::new()),
                    ))
                }
                #[staticmethod]
                pub fn register_bash_program(
                    code: &str,
                    entrypoint: &str,
                    arg_functions: Vec<(Vec<&str>, &str)>,
                    kwarg_functions: HashMap<&str, (Vec<&str>, &str)>,
                ) -> PyResult<[<$element Program>]> {
                    Ok([<$element Program>]::new(
                        code, 
                        entrypoint, 
                        arg_functions, 
                        kwarg_functions, 
                        Dialect::Bash(aorist_core::Bash::new()),
                    ))
                }
            }
            impl <'a> TProgram<'a, $element> for [<$element Program>] {
                fn new(
                    code: AString,
                    entrypoint: AString,
                    arg_functions: AVec<(AVec<AString>, AString)>,
                    kwarg_functions: LinkedHashMap<AString, (AVec<AString>, AString)>,
                    dialect: Dialect,
                ) -> Self {
                    Self {
                        code,
                        entrypoint,
                        arg_functions: arg_functions.clone().into_iter()
                            .map(|(x, y)| (x.into_iter().collect(), y)).collect(),
                        kwarg_functions: kwarg_functions.clone().into_iter().map(
                            |(k, (v, x))| (k, (v.into_iter().collect(), x))
                        ).collect(),
                        dialect
                    }
                }
                fn get_arg_functions(&self) -> AVec<(AVec<AString>, AString)> {
                    self.arg_functions.clone().into_iter().map(|(x, y)| (x.into_iter().collect(), y)).collect()
                }
                fn get_code(&self) -> AString {
                    self.code.clone()
                }
                fn get_dialect(&self) -> Dialect {
                    self.dialect.clone()
                }
                fn get_entrypoint(&self) -> AString {
                    self.entrypoint.clone()
                }
                fn get_kwarg_functions(&self) -> LinkedHashMap<AString, (AVec<AString>, AString)> {
                    self.kwarg_functions.clone().into_iter().map(
                        |(k, (v, x))| (k, (v.into_iter().collect(), x))
                    ).collect()
                }
            }
            impl $element {
                // TODO: move any of these functions that should have public visibility
                // into TConstraint
                fn get_uuid(&self) -> Result<AUuid> {
                    Ok(self.id.clone())
                }
                fn _should_add(root: AoristRef<Concept>, ancestry: &ConceptAncestry) -> bool {
                    $should_add(root, ancestry)
                }
                fn get_required(root: AoristRef<Concept>, ancestry: &ConceptAncestry) -> AVec<AUuid> {
                    $get_required(root, ancestry).into_iter().collect()
                }
                fn get_root_uuid(&self) -> Result<AUuid> {
                    Ok(self.root_uuid.clone())
                }
                fn requires_program(&self) -> Result<bool> {
                    Ok($requires_program)
                }
                // these are *all* downstream constraints
                fn get_downstream_constraints(&self) -> Result<AVec<RArc<RRwLock<Constraint>>>> {
                    let mut downstream: AVec<RArc<RRwLock<Constraint>>> = AVec::new();
                    $(
                        for arc in self.[<$required:snake:lower>].iter() {
                            downstream.push(arc.clone());
                        }
                    )*
                    Ok(downstream)
                }
                fn get_title() -> AOption<AString> {
                    $title
                }
                fn get_body() -> AOption<AString> {
                    $body
                }
            }
            impl <'a> TConstraint<'a> for $element {
                type Root = AoristRef<$root>;
                type Outer = $outer;
                type Ancestry = ConceptAncestry;

                fn get_root_type_name() -> Result<AString> {
                    Ok(stringify!($root).into())
                }
                fn get_required_constraint_names() -> AVec<AString> {
                    vec![$(
                        stringify!($required).into()
                    ),*].into_iter().collect()
                }
                fn should_add(root: AoristRef<Concept>, ancestry: &ConceptAncestry) -> bool {
                    let read = root.0.read();
                    match &*read {
                        Concept::$root(_) => Self::_should_add(root.clone(), ancestry),
                        _ => panic!("should_add called with unexpected concept."),
                    }
                }
                fn new(root_uuid: AUuid,
                       potential_child_constraints: AVec<RArc<RRwLock<Constraint>>>) -> Result<Self> {
                    // TODO: we should dedupe potential child constraints
                    $(
                        let mut [<$required:snake:lower>]: AVec<RArc<RRwLock<Constraint>>> =
                        AVec::new();
                    )*
                    let mut by_uuid: HashMap<AUuid, RArc<RRwLock<Constraint>>> = HashMap::new();
                    for constraint in potential_child_constraints.iter() {
                        $(
                            if let Some(AoristConstraint::$required{..}) =
                            &constraint.read().inner
                            {
                                by_uuid.insert(
                                    constraint.read().get_uuid()?,
                                    constraint.clone()
                                );
                            }
                        )*
                    }
                    for constraint in by_uuid.values() {
                        $(
                            if let Some(AoristConstraint::$required{..}) =
                            &constraint.read().inner {
                                [<$required:snake:lower>].push(constraint.clone());
                            }
                        )*
                    }
                    Ok(Self{
                        id: AUuid::new_v4(),
                        root_uuid,
                        $([<$required:snake:lower>]: [<$required:snake:lower>].into_iter().collect(),)*
                    })
                }
            }
        }
    };
}

#[macro_export]
macro_rules! register_attribute_new {
    ( $name:ident, $($element: ident),+ ) => { paste! {
        #[repr(C)]
        #[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, abi_stable::StableAbi)]
        pub enum [<$name Enum>] {
            $(
                $element($element),
            )+
        }
        #[cfg(feature = "python")]
        impl <'a> FromPyObject<'a> for [<$name Enum>] {
            fn extract(obj: &'a PyAny) -> PyResult<Self> {
                $(
                    if let Ok(x) = $element::extract(obj) {
                        return Ok(Self::$element(x));
                    }
                )+
                Err(PyValueError::new_err("could not convert enum arm."))
            }
        }
        #[cfg(feature = "python")]
        impl IntoPy<PyObject> for [<$name Enum>] {
            fn into_py(self, py: Python) -> PyObject {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.into_py(py),
                    )+
                }
            }
        }
        impl [<$name Enum>] {
            pub fn get_name(&self) -> AString {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_name(),
                    )+
                }
            }
            pub fn get_type(&self) -> AString {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => stringify!($element).into(),
                    )+
                }
            }
            pub fn is_nullable(&self) -> bool {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.is_nullable(),
                    )+
                }
            }
            pub fn is_key_type(&self) -> bool {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => $element::is_key_type(),
                    )+
                }
            }

            pub fn as_predicted_objective(&self) -> Self {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => [<$name Enum>]::$element($element {
                            name: format!("predicted_{}", x.get_name().as_str()).as_str().into(),
                            comment: x.get_comment().clone(),
                            nullable: false,
                        }),
                    )+
                }
            }
            pub fn get_comment(&self) -> AOption<AString> {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_comment(),
                    )+
                }
            }
            #[cfg(feature = "sql")]
            pub fn get_sql_type(&self) -> sqlparser::ast::DataType {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_sql_type(),
                    )+
                }
            }
            pub fn get_presto_type(&self) -> AString {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_presto_type(),
                    )+
                }
            }
            pub fn get_sqlite_type(&self) -> AString {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_sqlite_type(),
                    )+
                }
            }
            pub fn get_postgres_type(&self) -> AString {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_postgres_type(),
                    )+
                }
            }
            pub fn psycopg2_value_json_serializable(&self) -> bool {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.psycopg2_value_json_serializable(),
                    )+
                }
            }
            pub fn get_bigquery_type(&self) -> AString {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_bigquery_type(),
                    )+
                }
            }
            pub fn get_orc_type(&self) -> AString {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_orc_type(),
                    )+
                }
            }
            #[cfg(feature = "python")]
            pub fn get_py_type(&self) -> PyResult<pyo3::prelude::PyObject> {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_py_type(),
                    )+
                }
            }
        }
        #[aorist]
        pub struct $name {
            pub inner: AttributeOrTransform,
        }
        /*#[cfg(feature = "python")]
        impl<'a> FromPyObject<'a> for $name {
            fn extract(ob: &'a PyAny) -> PyResult<Self> {
                let inner = AttributeOrTransform::extract(ob)?;
                Ok(Self{ inner, tag: None, uuid: None })
            }
        }*/
        impl $name {
            pub fn get_name(&self) -> AString {
                self.inner.get_name()
            }
            pub fn psycopg2_value_json_serializable(&self) -> bool {
                self.inner.psycopg2_value_json_serializable()
            }
            pub fn get_type(&self) -> AString {
                self.inner.get_type()
            }
            pub fn is_nullable(&self) -> bool {
                self.inner.is_nullable()
            }
            pub fn is_key_type(&self) -> bool {
                self.inner.is_key_type()
            }
            pub fn get_comment(&self) -> AOption<AString> {
                self.inner.get_comment()
            }
            #[cfg(feature = "sql")]
            pub fn get_sql_type(&self) -> DataType {
                self.inner.get_sql_type()
            }
            pub fn get_presto_type(&self) -> AString {
                self.inner.get_presto_type()
            }
            pub fn get_sqlite_type(&self) -> AString {
                self.inner.get_sqlite_type()
            }
            pub fn get_postgres_type(&self) -> AString {
                self.inner.get_postgres_type()
            }
            pub fn get_bigquery_type(&self) -> AString {
                self.inner.get_bigquery_type()
            }
            pub fn get_orc_type(&self) -> AString {
                self.inner.get_orc_type()
            }
            #[cfg(feature = "python")]
            pub fn get_py_type(&self) -> PyResult<pyo3::prelude::PyObject> {
                self.inner.get_py_type()
            }
        }
        #[cfg(feature = "python")]
        aorist_paste::item!(
            pub fn [<$name:snake:lower>] (m: &PyModule) -> PyResult<()> {
                $(
                    m.add_class::<$element>()?;
                )+
                Ok(())
            }
        );
        #[cfg(feature = "python")]
        #[pyo3::prelude::pymethods]
        impl [<Py $name>] {
            #[getter]
            pub fn name(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().get_name().as_str().into())
            }
            #[getter]
            pub fn aorist_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().get_type().as_str().into())
            }
            #[getter]
            pub fn comment(&self) -> pyo3::prelude::PyResult<Option<String>> {
                Ok(
                    match self.inner.0.read().get_comment().clone() {
                        AOption(ROption::RSome(x)) => Some(x.as_str().into()),
                        AOption(ROption::RNone) => None,
                    }
                )
            }
            #[getter]
            pub fn orc_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().get_orc_type().as_str().into())
            }
            #[getter]
            pub fn presto_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().get_presto_type().as_str().into())
            }
            #[getter]
            pub fn bigquery_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().get_bigquery_type().as_str().into())
            }
            #[getter]
            pub fn sqlite_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().get_sqlite_type().as_str().into())
            }
            #[getter]
            pub fn is_nullable(&self) -> pyo3::prelude::PyResult<bool> {
                Ok(self.inner.0.read().is_nullable().clone())
            }
            #[getter]
            pub fn is_key(&self) -> bool {
                self.inner.0.read().is_key_type()
            }
            #[getter]
            pub fn psycopg2_value_json_serializable(&self) -> pyo3::prelude::PyResult<bool> {
                Ok(self.inner.0.read().psycopg2_value_json_serializable().clone())
            }
            #[getter]
            pub fn postgres_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().get_postgres_type().as_str().into())
            }
            #[getter]
            pub fn py_type(&self) -> PyResult<pyo3::prelude::PyObject> {
                self.inner.0.read().get_py_type()
            }
        }
    }}
}

#[macro_export]
macro_rules! register_concept {
    ( $name:ident, $ancestry:ident, $($element: ident ),* ) => { aorist_paste::item! {
        #[repr(C)]
        #[derive(Clone, Debug, Serialize, PartialEq, abi_stable::StableAbi)]
        pub enum $name {
            $(
                $element(AConcept<$element>),
            )+
        }
        impl ConceptEnum for $name {
            fn uuid(&self) -> AOption<AUuid> {
                self.get_uuid()
            }
        }
        impl AoristConceptBase for $name {
              type TChildrenEnum = $name;
              #[cfg(feature = "python")]
              fn py_object(inner: AoristRef<$name>, py: Python) -> Result<Py<pyo3::PyAny>, pyo3::PyErr> {
                  let object = match &*inner.0.read() {
                      $(
                          $name::$element(x) => PyObject::from(PyCell::new(py, [<Py $element>] {
                              inner: x.get_reference(),
                          }).unwrap()),
                      )+
                  };
                  Ok(object)
              }
              fn get_uuid(&self) -> AOption<AUuid> {
                  match &self {
                      $(
                        $name::$element(x) => x.get_own_uuid(),
                      )*
                  }
              }
              fn set_uuid(&mut self, uuid: AUuid) {
                  match self {
                      $(
                        $name::$element(ref mut x) => x.set_uuid(uuid),
                      )*
                  }
              }
              fn deep_clone(&self) -> Self {
                  match &self {
                      $(
                        $name::$element(x) => $name::$element(x.deep_clone()),
                      )*
                  }
              }
              fn get_tag(&self) -> AOption<AString> {
                  match self {
                      $(
                        $name::$element(x) => x.get_tag(),
                      )*
                  }
              }
              fn compute_uuids(&mut self) {
                  match self {
                      $(
                        $name::$element(x) => x.compute_uuids(),
                      )*
                  }
              }
              fn get_children(&self) -> AVec<(
                  // enum name
                  AString,
                  // field name
                  AOption<AString>,
                  // ix
                  AOption<usize>,
                  // uuid
                  AOption<AUuid>,
                  Self,
              )> {
                  vec![(
                      stringify!($name).into(),
                      AOption(ROption::RNone),
                      AOption(ROption::RNone),
                      self.get_uuid(),
                      self.clone(),
                  )].into_iter().collect()
              }
        }
        #[cfg(feature = "python")]
        pub fn concept_module(py: pyo3::prelude::Python, m: &pyo3::prelude::PyModule) -> pyo3::prelude::PyResult<()> {
            $(
                m.add_class::<[<Py $element>]>()?;
            )+
            m.add_class::<$ancestry>()?;
            Ok(())
        }
        $(
            impl [<CanBe $element>] for $name {
                fn [<construct_ $element:snake:lower>](
                    obj_ref: AoristRef<$element>,
                    ix: AOption<usize>,
                    id: AOption<(AUuid, AString)>
                ) -> AoristRef<Self> {
                    AoristRef(RArc::new(RRwLock::new($name::$element(AConcept::<$element>::new(
                        obj_ref.clone(),
                        match ix {
                            AOption(ROption::RSome(i)) => i,
                            AOption(ROption::RNone) => 0,
                        },
                        id,
                    )))))
               }
            }
        )+

        pub trait [<$name Descendants>] {
            fn get_descendants(&self) -> AVec<AoristRef<$name>>;
        }

        $(
            impl [<$name Descendants>] for AoristRef<$element> {
                fn get_descendants(&self) -> AVec<AoristRef<$name>> {
                    let mut concepts = AVec::new();
                    for tpl in self.get_children() {
                        let (name, field, ix, uuid, children_enum) = tpl;
                        let converted = children_enum.convert(name, field, ix, uuid);
                        concepts.push(converted);
                    }
                    concepts
                }
            }
            impl [<$name Descendants>] for AConcept<$element> {
                fn get_descendants(&self) -> AVec<AoristRef<$name>> {
                    self.get_reference().get_descendants()
                }
            }
        )+


        #[cfg_attr(feature = "python", pyclass(module = "aorist"))]
        pub struct $ancestry {
            pub parents: RArc<RRwLock<HashMap<(AUuid, AString), AoristRef<$name>>>>,
        }
        impl Ancestry for $ancestry {
            type TConcept = AoristRef<$name>;
            fn new(parents: RArc<RRwLock<HashMap<(AUuid, AString), AoristRef<$name>>>>) -> Self {
                 Self { parents }
            }
            fn get_parents(&self) -> RArc<RRwLock<HashMap<(AUuid, AString), AoristRef<$name>>>> {
                self.parents.clone()
            }

        }
        impl $ancestry {
            $(
                pub fn [<$element:snake:lower>](
                    &self,
                    root: AoristRef<$name>,
                ) -> Result<AoristRef<$element>, String> {
                    if root.get_type().as_str() == stringify!($element) {
                        let read = root.0.read();
                        return match *read {
                            $name::$element(ref x) => Ok(x.get_reference()),
                            _ => Err("Cannot convert.".into()),
                        };
                    }
                    let parent_id = root.get_parent_id();
                    match parent_id {
                        AOption(ROption::RNone) => Err(
                            format!(
                                "Cannot find ancestor of type {} for root {}.",
                                stringify!($element),
                                root.get_type(),
                            )
                        ),
                        AOption(ROption::RSome(id)) => {
                            let guard = self.parents.read();
                            let parent = guard.get(&id).unwrap().clone();
                            self.[<$element:snake:lower>](parent)
                        }
                    }
                }
            )+
        }
        #[cfg(feature = "python")]
        impl $ancestry {
            pub fn py_object(&self, ancestor: &str, root: AoristRef<$name>, py: Python) -> PyResult<PyObject> {
                match ancestor {
                    $(
                        stringify!([<$element:snake:lower>]) => match self.[<$element:snake:lower>](root) {
                            Ok(x) => x.py_object(py),
                            Err(err) => Err(pyo3::exceptions::PyTypeError::new_err(err.clone())),
                        }
                    )+
                    _ => panic!("Unknown ancestor type: {}", ancestor),
                }
            }
        }

        impl ToplineConceptBase for $name {
            type TUniverse = AoristRef<Universe>;
            fn build_universe(universe: AoristRef<Universe>) -> Self {
                $name::Universe(AConcept::new(universe, 0, AOption(ROption::RNone)))
            }
            fn get_parent_id(&self) -> AOption<(AUuid, AString)> {
                match self {
                    $(
                        $name::$element(ref x) => x.get_parent_id(),
                    )+
                }
            }
            fn get_type(&self) -> AString {
                match self {
                    $(
                        $name::$element(x) => stringify!($element).into(),
                    )*
                }
            }
            fn get_index_as_child(&self) -> usize {
                match self {
                    $(
                        $name::$element(x) => x.get_index_as_child(),
                    )*
                }
            }
            fn get_child_concepts(&self) -> AVec<AoristRef<Self>> {
                match self {
                    $(
                        $name::$element(ref x) => x.get_descendants(),
                    )*
                }
            }
            fn populate_child_concept_map(&self, concept_map: &mut HashMap<(AUuid, AString), AoristRef<Self>>) {
                match self {
                    $(
                        $name::$element(x) => {
                            let parent = x.get_parent_id();
                            let idx = x.get_index_as_child();
                            debug!("Visiting concept {}: {}", stringify!($element), x.get_uuid().unwrap());
                            for child in x.get_descendants() {
                                child.populate_child_concept_map(concept_map);
                            }
                            concept_map.insert(
                                (
                                    x.get_own_uuid().unwrap(),
                                    stringify!($element).into()
                                 ),
                                 AoristRef(RArc::new(RRwLock::new(
                                    $name::$element(x.clone())
                                 ))),
                            );
                        }
                    )*
                }
            }
        }
    }
    }
}
#[macro_export]
macro_rules! register_constraint {
    ( $name:ident, $lt: lifetime, $($element: ident),+ ) => { aorist_paste::item! {
        #[sabi_extern_fn]
        pub fn builders() -> RResult<AVec<RString>, AoristError> {
            ROk(vec![$(stringify!($element).into()),+].into_iter().collect().into())
        }
    }}
}
#[macro_export]
macro_rules! register_constraint_new {
    ( $name:ident, $lt: lifetime, $($element: ident),+ ) => { aorist_paste::item! {
        #[derive(Clone)]
        pub enum $name {
            $(
                $element($element),
            )+
        }
        #[cfg_attr(feature = "python", pyclass(module = "aorist"))]
        #[derive(Clone)]
        pub struct [<$name Program>] {
            inner: [<$name ProgramEnum>],
        }

        #[cfg(feature = "python")]
        #[pymethods]
        impl [<$name Program>] {
            #[new]
            fn new(inner: [<$name ProgramEnum>]) -> Self {
                Self { inner }
            }
        }
        #[cfg(feature = "python")]
        impl TOuterProgram for [<$name Program>] {
            type TAncestry = ConceptAncestry;
            fn get_dialect(&self) -> Dialect {
                self.inner.get_dialect()
            }
            fn compute_args<'a, T: aorist_core::OuterConstraint<'a>>(
                &self,
                root: <Self::TAncestry as Ancestry>::TConcept,
                ancestry: &Self::TAncestry,
                context: &mut aorist_primitives::Context,
                constraint: abi_stable::std_types::RArc<abi_stable::external_types::parking_lot::rw_lock::RRwLock<T>>,
            ) -> (AString, AString, ParameterTuple, Dialect) {
                let gil = Python::acquire_gil();
                let py = gil.python();
                //let mut args = AVec::new();
                let dill: &PyModule = PyModule::import(py, "dill").unwrap();
                let mut args: AVec<AST> = AVec::new();
                let mut kwargs: LinkedHashMap<AString, AST> = LinkedHashMap::new();
                for (input_types, serialized) in self.inner.get_arg_functions().iter() {
                    let py_arg = PyString::new(py, serialized.as_str());
                    let deserialized = dill.getattr("loads").unwrap().call1((py_arg,)).unwrap();
                    let mut objects = Vec::new();
                    let mut context_pos = None;
                    for (i, x) in input_types.iter().enumerate() {
                        if x.as_str() == "context" {
                            assert!(context_pos.is_none());
                            context_pos = Some(i);
                        } else {
                            objects.push(
                                ancestry.py_object(x.as_str(), root.clone(), py).unwrap().to_object(py)
                            );
                        }
                    }
                    let extracted;
                    if let Some(pos) = context_pos {
                        let obj = PyObject::from(PyCell::new(py, context.clone()).unwrap());
                        objects.insert(pos, obj.to_object(py));
                        let returned = deserialized.call1((objects,)).unwrap();
                        let (
                            extracted_string, extracted_context
                        ) : (String, aorist_primitives::Context) = returned.extract().unwrap();
                        context.insert(&extracted_context, constraint.read().get_name().as_str());
                        extracted = extracted_string;
                    } else {
                        let arg = deserialized.call1((objects,)).unwrap();
                        // TODO: add more return types here
                        extracted = arg.extract().unwrap();
                    }
                    let ast = AST::StringLiteral(StringLiteral::new_wrapped(extracted.as_str().into(), false));
                    args.push(ast);
                }
                for (key, (input_types, serialized)) in &self.inner.get_kwarg_functions() {
                    let py_arg = PyString::new(py, serialized.as_str());
                    let py_arg = py_arg.call_method1("encode", ("latin-1",)).unwrap();
                    let deserialized = dill.getattr("loads").unwrap().call1((py_arg,)).unwrap();


                    let mut objects = Vec::with_capacity(input_types.len());
                    let mut context_pos = None;
                    let mut constraint_pos = None;
                    for (i, x) in input_types.iter().enumerate() {
                        match x.as_str() {
                            "constraint" => {
                                assert!(constraint_pos.is_none());
                                constraint_pos = Some(i);
                                let constraint_rw = constraint.read();
                                let inner = constraint_rw.inner(stringify!($name)).unwrap();
                                let obj = inner.get_py_obj(py);
                                objects.push(obj);
                            },
                            "context" => {
                                assert!(context_pos.is_none());
                                let obj = PyObject::from(PyCell::new(py, context.clone()).unwrap());
                                objects.push(obj);
                                context_pos = Some(i);
                            },
                            _ => match ancestry.py_object(x.as_str(), root.clone(), py) {
                                Ok(x) => objects.push(x.to_object(py)),
                                Err(err) => panic!(
                                    "Error when running program for key {} input_type {} # {}:\n{}",
                                    key, i, x, err,
                                ),
                            }
                        }
                    }
                    let extracted: AST = match deserialized.call1((objects,)) {
                        Ok(arg) => {
                            let result = match context_pos {
                                Some(_) => aorist_ast::extract_arg_with_context(arg, context, constraint.read().get_name().as_str()),
                                None => aorist_ast::extract_arg(arg),
                            };
                            match result {
                                Ok(x) => x,
                                Err(err) => {
                                    err.print(py);
                                    panic!("Problem when extracting key {}", key);
                                }
                            }
                        }
                        Err(err) => {
                            err.print(py);
                            panic!("Problem when extracting object (tag: {:?}). See traceback above",
                                   aorist_primitives::ToplineConcept::get_tag(&root));
                        }
                    };

                    if key.as_str().as_bytes()[0] != '_' as u8 {
                        kwargs.insert(key.clone(), extracted);
                    }
                }
                (
                    self.inner.get_code(),
                    self.inner.get_entrypoint(),
                    ParameterTuple { args, kwargs },
                    // TODO: this should be handled by self.inner.get_dialect()
                    self.inner.get_dialect(),
                )
            }
        }
        #[cfg_attr(feature = "python", derive(pyo3::prelude::FromPyObject))]
        #[derive(Clone)]
        pub enum [<$name ProgramEnum>] {
            $(
                $element([<$element Program>]),
            )+
        }
        impl [<$name ProgramEnum>] {
            #[cfg(feature = "python")]
            pub fn get_arg_functions(&self) -> AVec<(AVec<AString>, AString)> {
                match self {
                    $(
                        [<$name ProgramEnum>]::$element(x) => x.get_arg_functions(),
                    )+
                }
            }
            pub fn get_dialect(&self) -> Dialect {
                match self {
                    $(
                        [<$name ProgramEnum>]::$element(x) => x.get_dialect(),
                    )+
                }
            }
            pub fn get_code(&self) -> AString {
                match self {
                    $(
                        [<$name ProgramEnum>]::$element(x) => x.get_code(),
                    )+
                }
            }
            pub fn get_entrypoint(&self) -> AString {
                match self {
                    $(
                        [<$name ProgramEnum>]::$element(x) => x.get_entrypoint(),
                    )+
                }
            }
            pub fn get_kwarg_functions(&self) -> LinkedHashMap<AString, (AVec<AString>, AString)> {
                match self {
                    $(
                        [<$name ProgramEnum>]::$element(x) => x.get_kwarg_functions(),
                    )+
                }
            }
        }
        pub enum [<$name Builder>]<$lt> {
            $(
                $element(ConstraintBuilder<$lt, $element>),
            )+
        }
        #[cfg(feature = "python")]
        #[pymodule]
        fn libaorist_constraint(py: Python, m: &PyModule) -> PyResult<()> {
            init_logging();
            $(
                m.add_class::<$element>()?;
                m.add_class::<[<$name Program>]>()?;
            )+
            Ok(())
        }
        impl <$lt> TBuilder<$lt> for [<$name Builder>]<$lt> {
            type OuterType = Constraint;
            type TEnum = AoristRef<Concept>;
            type TAncestry = ConceptAncestry;
            fn builders() -> AVec<[<$name Builder>]<$lt>> where Self : Sized {
                vec![
                    $(
                        [<$name Builder>]::$element(
                            ConstraintBuilder::<$lt, $element>{
                                _phantom: std::marker::PhantomData,
                                _phantom_lt: std::marker::PhantomData,
                            }
                        ),
                    )+
                ].into_iter().collect()
            }
            fn get_constraint_name(&self) -> AString {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => stringify!($element).into(),
                    )+
                }
            }
            fn get_required_constraint_names(&self) -> AVec<AString> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_required_constraint_names(),
                    )+
                }
            }
            fn build_constraint(
                &self,
                root_uuid: AUuid,
                potential_child_constraints: AVec<RArc<RRwLock<Self::OuterType>>>,
            ) -> Result<Self::OuterType> {
                match &self {
                    $(
                        [<$name Builder>]::$element(x) => Ok(Constraint {
                            name: self.get_constraint_name(),
                            root: self.get_root_type_name()?,
                            requires: Some(self.get_required_constraint_names()),
                            inner: Some(
                                $name::$element(x.build_constraint(
                                    root_uuid,
                                    potential_child_constraints,
                                )?)
                            ),
                        }),
                    )+
                }
            }
            fn get_root_type_name(&self) -> Result<AString> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_root_type_name(),
                    )+
                }
            }
            fn get_required(&self, root: AoristRef<Concept>, ancestry:&ConceptAncestry) -> AVec<AUuid> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) =>
                        $element::get_required(root, ancestry),
                    )+
                }
            }
            fn should_add(&self, root: AoristRef<Concept>, ancestry:&ConceptAncestry) -> bool {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) =>
                        $element::should_add(root, ancestry),
                    )+
                }
            }
        }
        impl <$lt> TConstraintEnum<$lt> for $name {
            fn get_required_constraint_names() -> HashMap<AString, AVec<AString>> {
                vec! [
                    $(
                        (stringify!($element).into(), $element::get_required_constraint_names()),
                    )+
                ].into_iter().collect()
            }
            #[cfg(feature = "python")]
            fn get_py_obj<'b>(&self, py: pyo3::Python<'b>) -> pyo3::prelude::PyObject {
                match &self {
                    $(
                        $name::$element(elem) => {
                            pyo3::prelude::PyObject::from(
                                pyo3::prelude::PyCell::new(
                                    py, elem.clone()
                                ).unwrap()
                            )
                        }
                    )+
                }
            }
            fn get_explanations() -> HashMap<AString, (AOption<AString>, AOption<AString>)> {
                vec! [
                    $(
                        (stringify!($element).into(), (
                            $element::get_title(),
                            $element::get_body(),
                        )),
                    )+
                ].into_iter().collect()
            }
        }
        impl <$lt> $name {
            pub fn get_root_type_name(&self) -> Result<AString> {
                match self {
                    $(
                        Self::$element(_) => $element::get_root_type_name(),
                    )+
                }
            }
            pub fn get_downstream_constraints(&self) -> Result<AVec<RArc<RRwLock<Constraint>>>> {
                match self {
                    $(
                        Self::$element(x) => x.get_downstream_constraints(),
                    )+
                }
            }
            pub fn requires_program(&self) -> Result<bool> {
                match self {
                    $(
                        Self::$element(x) => x.requires_program(),
                    )+
                }
            }
            pub fn get_uuid(&self) -> Result<AUuid> {
                match self {
                    $(
                        Self::$element(x) => x.get_uuid(),
                    )+
                }
            }
            pub fn get_title(&self) -> AOption<AString> {
                match self {
                    $(
                        Self::$element(_) => $element::get_title(),
                    )+
                }
            }
            pub fn get_body(&self) -> AOption<AString> {
                match self {
                    $(
                        Self::$element(_) => $element::get_body(),
                    )+
                }
            }
            pub fn get_root_uuid(&self) -> Result<AUuid> {
                match self {
                    $(
                        Self::$element(x) => x.get_root_uuid(),
                    )+
                }
            }
            fn get_root_type_names() -> Result<HashMap<AString, AString>> {
                Ok(vec![
                    $(
                        (
                            stringify!($element).into(), $element::get_root_type_name()?
                        ),
                    )+
                ].into_iter().collect())
            }
            pub fn get_name(&self) -> aorist_primitives::AString {
                match self {
                    $(
                        Self::$element(x) => stringify!($element).into(),
                    )+
                }
            }
            pub fn should_add(
                &self,
                root: AoristRef<Concept>,
                ancestry: &ConceptAncestry,
            ) -> bool {
                match &self {
                    $(
                        Self::$element(_) => $element::should_add(root,
                        ancestry),
                    )+
                }
            }
        }}
    }
}
#[macro_export]
macro_rules! define_dag_function {
    ($name:ident) => {
        #[pyfunction(dialect_preferences = "vec![
            Dialect::R(R::new()),
            Dialect::Python(aorist_core::Python::new(vec![])), 
            Dialect::Bash(Bash::new()), 
            Dialect::Presto(Presto::new())
        ]")]
        pub fn $name<'a>(
            mut universe: PyUniverse,
            constraints: Vec<String>,
            mode: &str,
            programs: BTreeMap<String, Vec<AoristConstraintProgram>>,
            dialect_preferences: Vec<Dialect>,
            dag_name: Option<String>,
        ) -> PyResult<String> {
            universe.compute_uuids();
            let programs_map = programs.into_iter().map(|(k, v)| (k.as_str().into(), v.into_iter().collect())).collect();
            let (output, _requirements) = match mode {
                "airflow" => PythonBasedDriver::<
                    AoristConstraintBuilder<'a>,
                    AirflowFlowBuilder<AoristRef<Universe>>,
                    AoristRef<Universe>,
                    AoristRef<Concept>,
                    ConceptAncestry,
                    AoristConstraintProgram,
                >::new(
                    universe.inner.clone(),
                    constraints.into_iter().map(|x| x.as_str().into()).collect(),
                    programs_map,
                    dialect_preferences.into_iter().collect(),
                    true,
                )
                .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
                .run(match dag_name {
                    Some(x) => AOption(ROption::RSome(x.as_str().into())),
                    None => AOption(ROption::RNone),
                }),
                "prefect" => PythonBasedDriver::<
                    AoristConstraintBuilder<'a>,
                    PrefectFlowBuilder<AoristRef<Universe>>,
                    AoristRef<Universe>,
                    AoristRef<Concept>,
                    ConceptAncestry,
                    AoristConstraintProgram,
                >::new(
                    universe.inner.clone(),
                    constraints.into_iter().map(|x| x.as_str().into()).collect(),
                    programs_map,
                    dialect_preferences.into_iter().collect(),
                    true,
                )
                .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
                .run(match dag_name {
                    Some(x) => AOption(ROption::RSome(x.as_str().into())),
                    None => AOption(ROption::RNone),
                }),
                "python" => PythonBasedDriver::<
                    AoristConstraintBuilder<'a>,
                    PythonFlowBuilder<AoristRef<Universe>>,
                    AoristRef<Universe>,
                    AoristRef<Concept>,
                    ConceptAncestry,
                    AoristConstraintProgram,
                >::new(
                    universe.inner.clone(),
                    constraints.into_iter().map(|x| x.as_str().into()).collect(),
                    programs_map,
                    dialect_preferences.into_iter().collect(),
                    false,
                )
                .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
                .run(match dag_name {
                    Some(x) => AOption(ROption::RSome(x.as_str().into())),
                    None => AOption(ROption::RNone),
                }),
                "jupyter" => PythonBasedDriver::<
                    AoristConstraintBuilder<'a>,
                    JupyterFlowBuilder<AoristRef<Universe>>,
                    AoristRef<Universe>,
                    AoristRef<Concept>,
                    ConceptAncestry,
                    AoristConstraintProgram,
                >::new(
                    universe.inner.clone(),
                    constraints.into_iter().map(|x| x.as_str().into()).collect(),
                    programs_map,
                    dialect_preferences.into_iter().collect(),
                    false,
                )
                .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
                .run(match dag_name {
                    Some(x) => AOption(ROption::RSome(x.as_str().into())),
                    None => AOption(ROption::RNone),
                }),
                /*"r" => RBasedDriver::<ConstraintBuilder, RBasedFlowBuilder>::new(&universe, constraints.into_iter().collect())
                .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
                .run(dag_name),*/
                _ => panic!("Unknown mode provided: {}", mode),
            }
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
            Ok(output.as_str().to_string().replace("\\\\", "\\").as_str().into())
        }
    }
}
#[macro_export]
macro_rules! export_aorist_python_module {
    ($module_name: ident, $dag_function: ident, $constraints_crate: ident, $attributes_crate: ident) => {
        use aorist_core::*;
        use aorist_primitives::*;
        use aorist_util::init_logging;
        use pyo3::prelude::*;
        use pyo3::wrap_pyfunction;
        use scienz::*;
        use std::collections::BTreeMap;
        use $attributes_crate::attributes_module;
        use $constraints_crate::*;

        use abi_stable::library::{lib_header_from_path, LibrarySuffix, RawLibrary};
        use abi_stable::reexports::SelfOps;
        use abi_stable::std_types::ROption;
        use aorist_core::ConstraintMod_Ref;
        use std::path::{Path, PathBuf};

        define_dag_function!($dag_function);
        #[pyfunction]
        pub fn test() -> PyResult<Vec<String>> {
            let base_name = "constraint_module";
            let debug_dir = "../target/debug/".as_ref_::<Path>().into_::<PathBuf>();
            let debug_path =
                RawLibrary::path_in_directory(&debug_dir, base_name, LibrarySuffix::NoSuffix);
            let header = lib_header_from_path(&debug_path).unwrap();
            let root_module = header.init_root_module::<ConstraintMod_Ref>().unwrap();
            let constructor = root_module.builders();
            Ok(constructor()
                .unwrap()
                .into_iter()
                .map(|x| x.into())
                .collect())
        }

        #[pymodule]
        fn $module_name(py: pyo3::prelude::Python, m: &PyModule) -> PyResult<()> {
            init_logging();
            attributes_module(py, m)?;
            constraints_module(py, m)?;
            concept_module(py, m)?;
            endpoints_module(py, m)?;
            dialects_module(py, m)?;
            m.add_wrapped(wrap_pyfunction!($dag_function))?;
            m.add_wrapped(wrap_pyfunction!(test))?;
            Ok(())
        }
    };
}
#[macro_export]
macro_rules! attribute {
    {$attribute: ident ( $name: expr, $comment: expr, $nullable: expr ) } => {
        Attribute {
            inner: AttributeOrTransform::Attribute(
                AoristRef(
                    abi_stable::std_types::RArc::new(
                        abi_stable::external_types::parking_lot::rw_lock::RRwLock::new(
                            AttributeEnum::$attribute($attribute {
                                name: $name,
                                comment: $comment,
                                nullable: $nullable,
                            })
                        )
                    )
                )
            ),
            tag: AOption(ROption::RNone),
            uuid: AOption(ROption::RNone),
        }
        //)))
    }
}
#[macro_export]
macro_rules! asset {
    { $name: ident } => {
        #[aorist]
        pub struct $name {
            pub name: AString,
            pub comment: AOption<AString>,
            #[constrainable]
            pub schema: AoristRef<DataSchema>,
            #[constrainable]
            pub setup: AoristRef<StorageSetup>,
        }
        impl TAsset for $name {
            fn get_name(&self) -> AString {
                self.name.clone()
            }
            fn get_schema(&self) -> AoristRef<DataSchema> {
                self.schema.clone()
            }
            fn get_storage_setup(&self) -> AoristRef<StorageSetup> {
                self.setup.clone()
            }
        }

        impl $name {
            pub fn set_storage_setup(&mut self, setup: AoristRef<StorageSetup>) {
                self.setup = setup;
            }
            pub fn replicate_to_local(
                &self,
                t: AoristRef<Storage>,
                tmp_dir: AString,
                tmp_encoding: AoristRef<Encoding>,
            ) -> Option<Self> {
                if let StorageSetup::RemoteStorageSetup(s) = &*self.setup.0.read() {
                    return Some(Self {
                        name: self.name.clone(),
                        comment: self.comment.clone(),
                        setup: AoristRef(RArc::new(RRwLock::new(
                            self.setup
                                .0
                                .read()
                                .replicate_to_local(t, tmp_dir, tmp_encoding),
                        ))),
                        schema: self.schema.clone(),
                        tag: self.tag.clone(),
                        uuid: AOption(ROption::RNone),
                    });
                }
                None
            }
        }
    }
}

#[macro_export]
macro_rules! derived_schema {
    {name: $name: ident
    $(, source: $source: ty)?
    $(, sources: $sources: ty)?
    $(, sources_map: BTreeMap<String, AoristRef<$sources_map: ty>>)?
    $(, sources:
        $(- $source_name: ident : $source_type: ty),+
    )?
    , attributes:
    $($attr_name: ident : $attribute: ident ($comment: expr, $nullable: expr )),+
    $(fields: $($field_name: ident : $field_type: ty),+)?
    } => { aorist_paste::paste! {
        #[aorist]
        pub struct $name {
            pub datum_template: AoristRef<DatumTemplate>,
            $(pub source: AoristRef<$source>,)?
            $(pub sources: AVec<AoristRef<$sources>>,)?
            $(pub sources_map: std::collections::BTreeMap<String, AoristRef<$sources_map>>,)?
            $($(
                pub $field_name: $field_type,
            )+)?
            $($(pub $source_name: AoristRef<$source_type>,)+)?
        }
        aorist_primitives::schema! {
            name: $name,
            attributes: $(
                $attr_name: $attribute($comment, $nullable)
            ),+
        }
        $(
            impl DerivedAssetSchema<'_> for $name {
                type SourceAssetType = $source;
            }
            impl SingleSourceDerivedAssetSchema<'_> for $name {
                fn get_source(&self) -> AoristRef<$source> {
                    self.source.clone()
                }
            }
        )?
        $(
            impl DerivedAssetSchema<'_> for $name {
                type SourceAssetType = $sources;
            }
            impl MultipleSourceDerivedAssetSchema<'_> for $name {
                fn get_sources(&self) -> AVec<Asset> {
                    self.sources.clone().into_iter().map(|x| Asset::$sources(x)).collect()
                }
            }
            #[cfg(feature = "python")]
            #[pymethods]
            impl [<Py $name>] {
                #[getter]
                pub fn sources(&self) -> Vec<PyAsset> {
                    self.inner.0.read().get_sources().into_iter().map(|x|
                        PyAsset{
                            inner: AoristRef(abi_stable::std_types::RArc::new(abi_stable::external_types::parking_lot::rw_lock::RRwLock::new(x)))
                        }
                    ).collect()
                }
            }
        )?
        $(
            impl DerivedAssetSchema<'_> for $name {
                type SourceAssetType = $sources_map;
            }
            impl MultipleSourceDerivedAssetSchema<'_> for $name {
                fn get_sources(&self) -> AVec<Asset> {
                    self.sources_map.values().map(|x| Asset::$sources_map(x.clone())).collect()
                }
            }
            #[cfg(feature = "python")]
            #[pymethods]
            impl [<Py $name>] {
                #[getter]
                pub fn sources(&self) -> Vec<PyAsset> {
                    self.inner.0.read().get_sources().into_iter().map(|x|
                        PyAsset{
                            inner: AoristRef(abi_stable::std_types::RArc::new(abi_stable::external_types::parking_lot::rw_lock::RRwLock::new(x)))
                        }
                    ).collect()
                }
            }
        )?
        $(
            #[cfg(feature = "python")]
            #[pymethods]
            impl [<Py $name>] {
                $(
                    #[getter]
                    pub fn [<get_ $source_name>](&self) -> [<Py $source_type>] {
                        [<Py $source_type>] {
                            inner: self.inner.0.read().$source_name.clone()
                        }
                    }
                )+
            }
        )?
     }}
}

#[macro_export]
macro_rules! primary_schema {
    {
        name: $name: ident
        $(, attributes:
            $($attr_name: ident : $attribute: ident ($comment: expr, $nullable: expr )),+
        )?
    } => { aorist_paste::paste! {
        #[aorist]
        pub struct $name {
            pub datum_template: AoristRef<DatumTemplate>,
        }
        aorist_primitives::schema! {
            name: $name
            $(, attributes: $(
                $attr_name: $attribute($comment, $nullable)
            ),+)?
        }
    }};
}
#[macro_export]
macro_rules! schema {
    {
        name: $name: ident
        $(, attributes: $(
            $attr_name: ident : $attribute: ident ($comment: expr, $nullable: expr )),+)?
    } => { aorist_paste::paste! {

        impl $name {
            pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
                vec![$($(
                    AoristRef(
                        abi_stable::std_types::RArc::new(
                            abi_stable::external_types::parking_lot::rw_lock::RRwLock::new(
                                attribute! { $attribute(
                                    stringify!($attr_name).into(),
                                    AOption(ROption::RSome($comment.into())),
                                    $nullable
                                )}
                            )
                        )
                    ),
                )+)?].into_iter().collect()
            }
            pub fn get_key(&self) -> AVec<AoristRef<Attribute>> {
                self.get_attributes().into_iter()
                    .filter(|x| x.0.read().is_key_type()).collect()
            }
            pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
                self.datum_template.clone()
            }
        }
        #[cfg(feature = "python")]
        #[pymethods]
        impl [<Py $name>] {
            #[getter]
            pub fn get_attributes(&self) -> Vec<PyAttribute> {
                self.inner.0.read().get_attributes().iter().map(|x| PyAttribute{ inner: x.clone() }).collect()
            }
            #[getter]
            pub fn get_key(&self) -> Vec<PyAttribute> {
                self.inner.0.read().get_key().iter().map(|x| PyAttribute{ inner: x.clone() }).collect()
            }
            #[getter]
            pub fn get_datum_template(&self) -> PyDatumTemplate {
                PyDatumTemplate{ inner: self.inner.0.read().get_datum_template() }
            }
        }
    }};
}
#[macro_export]
macro_rules! asset_enum {
    {
        name: $name: ident
        $($(concrete_variants)? $(variants)? : $(- $variant: ident)+)?
        $(enum_variants: $(- $enum_variant: ident)+)?
    } => { aorist_paste::paste! {

        #[aorist]
        #[derive(Eq)]
        pub enum $name {
            $($(
                #[constrainable]
                $variant(AoristRef<$variant>),
            )+)?
            $($(
                #[constrainable]
                $enum_variant(AoristRef<$enum_variant>),
            )+)?
        }
        impl TAsset for $name {
            fn get_name(&self) -> AString {
                match self {
                    $($(
                        Self::$variant(x) => x.0.read().name.clone(),
                    )+)?
                    $($(
                        Self::$enum_variant(x) => x.0.read().get_name(),
                    )+)?
                }
            }
            fn get_schema(&self) -> AoristRef<DataSchema> {
                match self {
                    $($(
                        Self::$variant(x) => x.0.read().get_schema(),
                    )+)?
                    $($(
                        Self::$enum_variant(x) => x.0.read().get_schema(),
                    )+)?
                }
            }
            fn get_storage_setup(&self) -> AoristRef<StorageSetup> {
                match self {
                    $($(
                        Self::$variant(x) => x.0.read().get_storage_setup(),
                    )+)?
                    $($(
                        Self::$enum_variant(x) => x.0.read().get_storage_setup(),
                    )+)?
                }
            }
        }
        impl $name {
            pub fn set_storage_setup(&mut self, setup: AoristRef<StorageSetup>) {
                match self {
                    $($(
                        Self::$variant(x) => x.0.write().set_storage_setup(setup),
                    )+)?
                    $($(
                        Self::$enum_variant(x) => x.0.write().set_storage_setup(setup),
                    )+)?
                }
            }
            pub fn get_type(&self) -> AString {
                match self {
                    $($(
                        Self::$variant(_) => stringify!($variant),
                    )+)?
                    $($(
                        Self::$enum_variant(_) => stringify!($enum_variant),
                    )+)?
                }
                .into()
            }
            pub fn replicate_to_local(
                &self,
                t: AoristRef<Storage>,
                tmp_dir: AString,
                tmp_encoding: AoristRef<Encoding>,
            ) -> Option<Self> {
                match self {
                    $($(
                        Self::$variant(x) => x.0.read()
                            .replicate_to_local(t, tmp_dir, tmp_encoding).and_then(|r|
                                Some(Self::$variant(AoristRef(RArc::new(RRwLock::new(r)))))
                            ),
                    )+)?
                    $($(
                        Self::$enum_variant(x) => x.0.read()
                            .replicate_to_local(t, tmp_dir, tmp_encoding).and_then(|r|
                                Some(Self::$enum_variant(AoristRef(RArc::new(RRwLock::new(r)))))
                            ),
                    )+)?
                }
            }
        }

        #[cfg(feature = "python")]
        #[pymethods]
        impl [<Py $name>] {
            #[getter]
            pub fn name(&self) -> AString {
                self.inner.0.read().get_name()
            }
            #[getter]
            pub fn get_storage_setup(&self) -> PyStorageSetup {
                PyStorageSetup {
                    inner: self.inner.0.read().get_storage_setup().clone(),
                }
            }
            #[getter]
            pub fn get_schema(&self) -> PyDataSchema {
                PyDataSchema {
                    inner: self.inner.0.read().get_schema().clone(),
                }
            }
        }
    }};
}
#[macro_export]
macro_rules! schema_enum {
    {
        name: $name: ident
        $($(concrete_variants)? $(variants)? : $(- $variant: ident)+)?
        $(enum_variants: $(- $enum_variant: ident)+)?
    } => { aorist_paste::paste! {

        #[aorist]
        #[derive(Eq)]
        pub enum $name {
            $($(
                #[constrainable]
                $variant(AoristRef<$variant>),
            )+)?
            $($(
                #[constrainable]
                $enum_variant(AoristRef<$enum_variant>),
            )+)?
        }
        impl $name {
            pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
                match self {
                    $($(
                        Self::$variant(x) => x.0.read().get_attributes(),
                    )+)?
                    $($(
                        Self::$enum_variant(x) => x.0.read().get_attributes(),
                    )+)?
                }
            }
            pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
                match self {
                    $($(
                        Self::$variant(x) => x.0.read().get_datum_template(),
                    )+)?
                    $($(
                        Self::$enum_variant(x) => x.0.read().get_datum_template(),
                    )+)?
                }
            }
        }
        #[cfg(feature = "python")]
        #[pymethods]
        impl [<Py $name>] {
            #[getter]
            pub fn get_datum_template(&self) -> PyDatumTemplate {
                PyDatumTemplate{ inner: self.inner.0.read().get_datum_template() }
            }
        }
    }};
}

#[macro_export]
macro_rules! define_constraint_abi {
    ($element:ident $(, $required:ident)*) => {
        aorist_paste::item! {

            #[repr(C)]
            #[cfg_attr(feature = "python", pyclass(module = "aorist"))]
            #[derive(Clone)]
            pub struct [<$element ABI>] {
                root: aorist_core::RefABI<aorist_core::Concept>,
            }
        }
    };
}

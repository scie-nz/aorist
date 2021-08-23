#![allow(non_snake_case)]

mod concept;
pub use concept::*;
mod endpoints;
pub use endpoints::*;
mod context;
pub use context::*;

#[macro_export]
macro_rules! register_ast_nodes {
    ($name:ident, $($variant: ident,)+) => {

        #[derive(Clone, Debug)]
        pub enum $name {
            $(
                $variant(Arc<RwLock<$variant>>),
            )+
        }
        impl $name {
            pub fn clone_without_ancestors(&self) -> Self {
                match &self {
                    $(
                        Self::$variant(x) => Self::$variant(Arc::new(RwLock::new(x.read().unwrap().clone_without_ancestors()))),
                    )+
                }
            }
            pub fn set_ancestors(&self, ancestors: Vec<AncestorRecord>) {
                match &self {
                    $(
                        Self::$variant(x) => x.write().unwrap().set_ancestors(ancestors),
                    )+
                }
            }
            pub fn get_ancestors(&self) -> Option<Vec<AncestorRecord>> {
                match &self {
                    $(
                        Self::$variant(x) => x.read().unwrap().get_ancestors(),
                    )+
                }
            }
            pub fn get_descendants(&self) -> Vec<AST> {
                let mut queue = VecDeque::new();
                queue.push_back(self.clone());
                let mut current = queue.pop_front();
                let mut v: Vec<AST> = Vec::new();
                while let Some(elem) = current {
                    let direct_descendants = match &elem {
                        $(
                            Self::$variant(x) => {
                            let read = x.read().unwrap();
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
                        Self::$variant(rw) => rw.write().unwrap().optimize_fields(),
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
                        Self::$variant(x) => x.read().unwrap().to_python_ast_node(
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
                        Self::$variant(x) => x.read().unwrap().to_r_ast_node(
                            depth,
                        ),
                    )+
                }
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                match (&self, other) {
                    $(
                        (Self::$variant(v1), Self::$variant(v2)) => {
                            v1.read().unwrap().eq(&v2.read().unwrap())
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
                        Self::$variant(x) => x.read().unwrap().hash(state),
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
            )*) -> Arc<RwLock<Self>> {
                Arc::new(RwLock::new(Self::new($($field, )*)))
            }
            pub fn get_statements<'a>(
                &self,
            ) -> Vec<AST> {
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
            pub fn get_direct_descendants(&self) -> Vec<AST> {
                $descendants(self)
            }
            pub fn get_imports(&self) -> Vec<$import_type> {
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
                $variant(Arc<RwLock<$variant>>),
            )+
        }
        impl $name {
            pub fn get_imports(&self) -> Vec<$import_type>{
                match &self {
                    $(
                        Self::$variant(x) => x.read().unwrap().get_imports(),
                    )+
                }
            }
            pub fn get_statements(&self) -> Vec<AST> {
                match &self {
                    $(
                        Self::$variant(x) => x.read().unwrap().get_statements(),
                    )+
                }
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                match (&self, other) {
                    $(
                        (Self::$variant(v1), Self::$variant(v2)) => {
                            v1.read().unwrap().eq(&v2.read().unwrap())
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
                        Self::$variant(x) => x.read().unwrap().hash(state),
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
            ancestors: Option<Vec<AncestorRecord>>,
        }
        impl $name {
            pub fn new_wrapped($(
                $field: $field_type,
            )*) -> Arc<RwLock<Self>> {
                Arc::new(RwLock::new(Self::new($($field, )*)))
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
                    ancestors: None,
                }
            }
            pub fn clone_without_ancestors(&self) -> Self {
                Self {
                    $($field: self.$field.clone(),)*
                    ancestors: None,
                }
            }
            pub fn set_ancestors(&mut self, ancestors: Vec<AncestorRecord>) {
                assert!(self.ancestors.is_none());
                self.ancestors = Some(ancestors);
            }
            pub fn get_ancestors(&self) -> Option<Vec<AncestorRecord>> {
                self.ancestors.clone()
            }
            $(
                pub fn $field(&self) -> $field_type {
                    self.$field.clone()
                }
            )*
            pub fn get_direct_descendants(&self) -> Vec<AST> {
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
                uuid: Uuid,
                c: Concept<'a>,
                ancestry: Arc<ConceptAncestry<'a>>,
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
      $value:ident
    ) => {
        aorist_paste::item! {
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
            )]
            #[cfg_attr(feature = "sql", derive($sql_type))]
            pub struct $element {
                pub name: String,
                pub comment: Option<String>,
                pub nullable: bool,
            }
            impl TAttribute for $element {
                type Value = $value;

                fn get_name(&self) -> &String {
                    &self.name
                }
                fn get_comment(&self) -> &Option<String> {
                    &self.comment
                }
                fn is_nullable(&self) -> bool {
                    self.nullable
                }
            }
            #[cfg(feature = "python")]
            #[pymethods]
            impl $element {
                #[new]
                #[args(comment = "None")]
                #[args(nullable = "false")]
                pub fn new(
                    name: String,
                    comment: Option<String>,
                    nullable: bool
                ) -> Self {
                    Self { name, comment, nullable }
                }
                #[getter]
                pub fn name(&self) -> PyResult<String> {
                    Ok(self.name.clone())
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
            #[cfg_attr(feature = "python", pyclass(module = "aorist"))]
            #[derive(Clone)]
            pub struct $element {
                id: Uuid,
                root_uuid: Uuid,
                $([<$required:snake:lower>] : Vec<Arc<RwLock<$outer>>>,)*
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
                    uuid: Uuid,
                    root: Concept,
                    ancestry: Arc<ConceptAncestry>,
                ) -> ParameterTuple;
                fn get_preamble() -> String;
                fn get_call() -> String;
                fn get_dialect() -> Dialect;
            }

            #[cfg_attr(feature = "python", pyclass(module = "aorist"))]
            #[derive(Clone)]
            pub struct [<$element Program>] {
                pub dialect: Dialect,
                pub code: String,
                pub entrypoint: String,
                pub arg_functions: Vec<(Vec<String>, String)>,
                pub kwarg_functions: LinkedHashMap<String, (Vec<String>, String)>,
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
                    pip_requirements: Vec<String>,
                ) -> PyResult<[<$element Program>]> {

                    let mut funs: LinkedHashMap<String, (Vec<String>, String)> = LinkedHashMap::new();
                    for (k, (v1, v2)) in kwarg_functions.iter() {
                        funs.insert(k.to_string(), (v1.iter().map(|x| x.to_string()).collect(), v2.to_string()));
                    }
                    Ok([<$element Program>]{
                        code: code.to_string(),
                        entrypoint: entrypoint.to_string(),
                        arg_functions: arg_functions.iter().map(|(x, y)| (x.iter().map(|x| x.to_string()).collect(), y.to_string())).collect(),
                        kwarg_functions: funs,
                        dialect: Dialect::Python(aorist_core::Python::new(pip_requirements))
                    })
                }
                #[staticmethod]
                pub fn register_r_program(
                    code: &str,
                    entrypoint: &str,
                    arg_functions: Vec<(Vec<&str>, &str)>,
                    kwarg_functions: HashMap<&str, (Vec<&str>, &str)>,
                ) -> PyResult<[<$element Program>]> {

                    let mut funs: LinkedHashMap<String, (Vec<String>, String)> = LinkedHashMap::new();
                    for (k, (v1, v2)) in kwarg_functions.iter() {
                        funs.insert(k.to_string(), (v1.iter().map(|x| x.to_string()).collect(), v2.to_string()));
                    }
                    Ok([<$element Program>]{
                        code: code.to_string(),
                        entrypoint: entrypoint.to_string(),
                        arg_functions: arg_functions.iter().map(|(x, y)| (x.iter().map(|x| x.to_string()).collect(), y.to_string())).collect(),
                        kwarg_functions: funs,
                        dialect: Dialect::R(aorist_core::R::new())
                    })
                }
                #[staticmethod]
                pub fn register_presto_program(
                    code: &str,
                    entrypoint: &str,
                    arg_functions: Vec<(Vec<&str>, &str)>,
                    kwarg_functions: HashMap<&str, (Vec<&str>, &str)>,
                ) -> PyResult<[<$element Program>]> {

                    let mut funs: LinkedHashMap<String, (Vec<String>, String)> = LinkedHashMap::new();
                    for (k, (v1, v2)) in kwarg_functions.iter() {
                        funs.insert(k.to_string(), (v1.iter().map(|x| x.to_string()).collect(), v2.to_string()));
                    }
                    Ok([<$element Program>]{
                        code: code.to_string(),
                        entrypoint: entrypoint.to_string(),
                        arg_functions: arg_functions.iter().map(|(x, y)| (x.iter().map(|x| x.to_string()).collect(), y.to_string())).collect(),
                        kwarg_functions: funs,
                        dialect: Dialect::Presto(aorist_core::Presto::new())
                    })
                }
                #[staticmethod]
                pub fn register_bash_program(
                    code: &str,
                    entrypoint: &str,
                    arg_functions: Vec<(Vec<&str>, &str)>,
                    kwarg_functions: HashMap<&str, (Vec<&str>, &str)>,
                ) -> PyResult<[<$element Program>]> {

                    let mut funs: LinkedHashMap<String, (Vec<String>, String)> = LinkedHashMap::new();
                    for (k, (v1, v2)) in kwarg_functions.iter() {
                        funs.insert(k.to_string(), (v1.iter().map(|x| x.to_string()).collect(), v2.to_string()));
                    }
                    Ok([<$element Program>]{
                        code: code.to_string(),
                        entrypoint: entrypoint.to_string(),
                        arg_functions: arg_functions.iter().map(|(x, y)| (x.iter().map(|x| x.to_string()).collect(), y.to_string())).collect(),
                        kwarg_functions: funs,
                        dialect: Dialect::Bash(aorist_core::Bash::new())
                    })
                }
            }
            impl <'a> TProgram<'a, $element> for [<$element Program>] {
                fn new(
                    code: String,
                    entrypoint: String,
                    arg_functions: Vec<(Vec<String>, String)>,
                    kwarg_functions: LinkedHashMap<String, (Vec<String>, String)>,
                    dialect: Dialect,
                ) -> Self {
                    Self { code, entrypoint, arg_functions, kwarg_functions, dialect }
                }
                fn get_arg_functions(&self) -> Vec<(Vec<String>, String)> {
                    self.arg_functions.clone()
                }
                fn get_code(&self) -> String {
                    self.code.clone()
                }
                fn get_dialect(&self) -> Dialect {
                    self.dialect.clone()
                }
                fn get_entrypoint(&self) -> String {
                    self.entrypoint.clone()
                }
                fn get_kwarg_functions(&self) -> LinkedHashMap<String, (Vec<String>, String)> {
                    self.kwarg_functions.clone()
                }
            }
            impl $element {
                // TODO: move any of these functions that should have public visibility
                // into TConstraint
                fn get_uuid(&self) -> Result<Uuid> {
                    Ok(self.id.clone())
                }
                fn _should_add(root: AoristRef<Concept>, ancestry: &ConceptAncestry) -> bool {
                    $should_add(root, ancestry)
                }
                fn get_required(root: AoristRef<Concept>, ancestry: &ConceptAncestry) -> Vec<Uuid> {
                    $get_required(root, ancestry)
                }
                fn get_root_uuid(&self) -> Result<Uuid> {
                    Ok(self.root_uuid.clone())
                }
                fn requires_program(&self) -> Result<bool> {
                    Ok($requires_program)
                }
                // these are *all* downstream constraints
                fn get_downstream_constraints(&self) -> Result<Vec<Arc<RwLock<Constraint>>>> {
                    let mut downstream: Vec<Arc<RwLock<Constraint>>> = Vec::new();
                    $(
                        for arc in &self.[<$required:snake:lower>] {
                            downstream.push(arc.clone());
                        }
                    )*
                    Ok(downstream)
                }
                fn get_title() -> Option<String> {
                    $title
                }
                fn get_body() -> Option<String> {
                    $body
                }
            }
            impl <'a> TConstraint<'a> for $element {
                type Root = AoristRef<$root>;
                type Outer = $outer;
                type Ancestry = ConceptAncestry;

                fn get_root_type_name() -> Result<String> {
                    Ok(stringify!($root).into())
                }
                fn get_required_constraint_names() -> Vec<String> {
                    vec![$(
                        stringify!($required).into()
                    ),*]
                }
                fn should_add(root: AoristRef<Concept>, ancestry: &ConceptAncestry) -> bool {
                    let read = root.0.read().unwrap();
                    match *read {
                        Concept::$root((_, _, _)) => Self::_should_add(root.clone(), ancestry),
                        _ => panic!("should_add called with unexpected concept."),
                    }
                }
                fn new(root_uuid: Uuid,
                       potential_child_constraints: Vec<Arc<RwLock<Constraint>>>) -> Result<Self> {
                    // TODO: we should dedupe potential child constraints
                    $(
                        let mut [<$required:snake:lower>]: Vec<Arc<RwLock<Constraint>>> =
                        Vec::new();
                    )*
                    let mut by_uuid: HashMap<Uuid, Arc<RwLock<Constraint>>> = HashMap::new();
                    for constraint in &potential_child_constraints {
                        $(
                            if let Some(AoristConstraint::$required{..}) =
                            &constraint.read().unwrap().inner
                            {
                                by_uuid.insert(
                                    constraint.read().unwrap().get_uuid()?,
                                    constraint.clone()
                                );
                            }
                        )*
                    }
                    for constraint in by_uuid.values() {
                        $(
                            if let Some(AoristConstraint::$required{..}) =
                            &constraint.read().unwrap().inner {
                                [<$required:snake:lower>].push(constraint.clone());
                            }
                        )*
                    }
                    Ok(Self{
                        id: Uuid::new_v4(),
                        root_uuid,
                        $([<$required:snake:lower>],)*
                    })
                }
            }
        }
    };
}

#[macro_export]
macro_rules! register_attribute_new {
    ( $name:ident, $($element: ident),+ ) => { paste! {
        #[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
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
            pub fn get_name(&self) -> &String {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_name(),
                    )+
                }
            }
            pub fn get_type(&self) -> String {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => stringify!($element).to_string(),
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

            pub fn as_predicted_objective(&self) -> Self {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => [<$name Enum>]::$element($element {
                            name: format!("predicted_{}", x.get_name()).to_string(),
                            comment: x.get_comment().clone(),
                            nullable: false,
                        }),
                    )+
                }
            }
            pub fn get_comment(&self) -> &Option<String> {
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
            pub fn get_presto_type(&self) -> String {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_presto_type(),
                    )+
                }
            }
            pub fn get_sqlite_type(&self) -> String {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_sqlite_type(),
                    )+
                }
            }
            pub fn get_postgres_type(&self) -> String {
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
            pub fn get_bigquery_type(&self) -> String {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_bigquery_type(),
                    )+
                }
            }
            pub fn get_orc_type(&self) -> String {
                match self {
                    $(
                        [<$name Enum>]::$element(x) => x.get_orc_type(),
                    )+
                }
            }
        }
        #[aorist(derivative(Hash))]
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
            pub fn get_name(&self) -> &String {
                self.inner.get_name()
            }
            pub fn psycopg2_value_json_serializable(&self) -> bool {
                self.inner.psycopg2_value_json_serializable()
            }
            pub fn get_type(&self) -> String {
                self.inner.get_type()
            }
            pub fn is_nullable(&self) -> bool {
                self.inner.is_nullable()
            }
            pub fn get_comment(&self) -> &Option<String> {
                self.inner.get_comment()
            }
            #[cfg(feature = "sql")]
            pub fn get_sql_type(&self) -> DataType {
                self.inner.get_sql_type()
            }
            pub fn get_presto_type(&self) -> String {
                self.inner.get_presto_type()
            }
            pub fn get_sqlite_type(&self) -> String {
                self.inner.get_sqlite_type()
            }
            pub fn get_postgres_type(&self) -> String {
                self.inner.get_postgres_type()
            }
            pub fn get_bigquery_type(&self) -> String {
                self.inner.get_bigquery_type()
            }
            pub fn get_orc_type(&self) -> String {
                self.inner.get_orc_type()
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
                Ok(self.inner.0.read().unwrap().get_name().clone())
            }
            #[getter]
            pub fn aorist_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().unwrap().get_type().clone())
            }
            #[getter]
            pub fn comment(&self) -> pyo3::prelude::PyResult<Option<String>> {
                Ok(self.inner.0.read().unwrap().get_comment().clone())
            }
            #[getter]
            pub fn orc_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().unwrap().get_orc_type().clone())
            }
            #[getter]
            pub fn presto_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().unwrap().get_presto_type().clone())
            }
            #[getter]
            pub fn bigquery_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().unwrap().get_bigquery_type().clone())
            }
            #[getter]
            pub fn sqlite_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().unwrap().get_sqlite_type().clone())
            }
            #[getter]
            pub fn is_nullable(&self) -> pyo3::prelude::PyResult<bool> {
                Ok(self.inner.0.read().unwrap().is_nullable().clone())
            }
            #[getter]
            pub fn psycopg2_value_json_serializable(&self) -> pyo3::prelude::PyResult<bool> {
                Ok(self.inner.0.read().unwrap().psycopg2_value_json_serializable().clone())
            }
            #[getter]
            pub fn postgres_type(&self) -> pyo3::prelude::PyResult<String> {
                Ok(self.inner.0.read().unwrap().get_postgres_type().clone())
            }
        }
    }}
}

#[macro_export]
macro_rules! register_concept {
    ( $name:ident, $ancestry:ident, $($element: ident ),* ) => { aorist_paste::item! {
        #[derive(Clone, Debug, Serialize, PartialEq)]
        pub enum $name {
            $(
                $element((AoristRef<$element>, usize, Option<(Uuid, String)>)),
            )+
        }
        #[cfg(feature = "python")]
        pub fn concept_module(py: pyo3::prelude::Python, m: &pyo3::prelude::PyModule) -> pyo3::prelude::PyResult<()> {
            $(
                m.add_class::<[<Py $element>]>()?;
            )+
            m.add_class::<$ancestry>()?;
            Ok(())
        }
        // note: both Universe and EndpointConfig must exist
        impl AoristUniverse for AoristRef<Universe> {
            type TEndpoints = EndpointConfig;
            fn get_endpoints(&self) -> Self::TEndpoints {
                (*self.0.read().unwrap()).endpoints.0.read().unwrap().clone()
            }
        }
        $(
            impl [<CanBe $element>] for $name {
                fn [<construct_ $element:snake:lower>](
                    obj_ref: AoristRef<$element>,
                    ix: Option<usize>,
                    id: Option<(Uuid, String)>
                ) -> AoristRef<Self> {
                    AoristRef(Arc::new(RwLock::new($name::$element((
                        obj_ref.clone(),
                        match ix {
                            Some(i) => i,
                            None => 0,
                        },
                        id,
                    )))))
               }
            }
        )+

        pub trait [<$name Descendants>] {
            fn get_descendants(&self) -> Vec<AoristRef<$name>>;
        }

        $(
            impl [<$name Descendants>] for AoristRef<$element> {
                fn get_descendants(&self) -> Vec<AoristRef<$name>> {
                    let mut concepts = Vec::new();
                    for tpl in self.get_children() {
                        let wrapped_concept = WrappedConcept::from(tpl);
                        concepts.push(wrapped_concept.inner);
                    }
                    concepts
                }
            }
        )+


        impl ConceptEnum for $name {}
        impl ConceptEnum for AoristRef<$name> {}

        $(
            impl TryFrom<AoristRef<$name>> for AoristRef<$element> {
                type Error = String;
                fn try_from(x: AoristRef<$name>) -> Result<Self, String> {
                    let read = x.0.read();
                    match read {
                        Ok(elem) => match *elem {
                            $name::$element((ref y, _, _)) => Ok(y.clone()),
                            _ => Err("Cannot convert.".into()),
                        },
                        _ => Err("Cannot read.".into()),
                    }
                }
            }
        )+

        #[cfg_attr(feature = "python", pyclass(module = "aorist"))]
        pub struct $ancestry {
            pub parents: Arc<RwLock<HashMap<(Uuid, String), AoristRef<$name>>>>,
        }
        impl Ancestry for $ancestry {
            type TConcept = AoristRef<$name>;
            fn new(parents: Arc<RwLock<HashMap<(Uuid, String), AoristRef<$name>>>>) -> Self {
                 Self { parents }
            }
            fn get_parents(&self) -> Arc<RwLock<HashMap<(Uuid, String), AoristRef<$name>>>> {
                self.parents.clone()
            }

        }
        impl $ancestry {
            $(
                pub fn [<$element:snake:lower>](
                    &self,
                    root: AoristRef<$name>,
                ) -> Result<AoristRef<$element>, String> {
                    if root.get_type() == stringify!($element).to_string(){
                        return(Ok(AoristRef::<$element>::try_from(root.clone()).unwrap()));
                    }
                    let parent_id = root.get_parent_id();
                    match parent_id {
                        None => Err(
                            format!(
                                "Cannot find ancestor of type {} for root {}.",
                                stringify!($element),
                                root.get_type(),
                            )
                        ),
                        Some(id) => {
                            let guard = self.parents.read().unwrap();
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
        #[cfg(feature = "python")]
        impl $name {
            pub fn py_object(&self, py: Python) -> PyObject {
                let object = match self {
                    $(
                        $name::$element((x, _, _)) => PyObject::from(PyCell::new(py, [<Py $element>] {
                            inner: x.clone(),
                        }).unwrap()),
                    )+
                };
                object
            }
        }
        impl TConceptEnum for AoristRef<$name> {
            type TUniverse = AoristRef<Universe>;
            fn get_parent_id(&self) -> Option<(Uuid, String)> {
                let read = self.0.read().unwrap();
                match *read {
                    $(
                        $name::$element((_, _, ref id)) => id.clone(),
                    )+
                }
            }
            fn from_universe(universe: AoristRef<Universe>) -> Self {
                AoristRef(Arc::new(RwLock::new($name::Universe((universe, 0, None)))))
            }
            fn get_type(&self) -> String {
                let read = self.0.read().unwrap();
                match *read {
                    $(
                        $name::$element((ref x, _, _)) => stringify!($element).to_string(),
                    )*
                }
            }
            fn get_uuid(&self) -> Uuid {
                let read = self.0.read().unwrap();
                match *read {
                    $(
                        $name::$element((ref x, _, _)) => x.get_uuid().unwrap(),
                    )*
                }
            }
            fn get_tag(&self) -> Option<String> {
                let read = self.0.read().unwrap();
                match *read {
                    $(
                        $name::$element((ref x, _, _)) => x.get_tag(),
                    )*
                }
            }
            fn get_index_as_child(&self) -> usize {
                let read = self.0.read().unwrap();
                match *read {
                    $(
                        $name::$element((_, idx, _)) => idx,
                    )*
                }
            }
            fn get_child_concepts(&self) -> Vec<Self> {
                let read = self.0.read().unwrap();
                match *read {
                    $(
                        $name::$element((ref x, _, _)) => x.get_descendants(),
                    )*
                }
            }
            fn populate_child_concept_map(&self, concept_map: &mut HashMap<(Uuid, String), Self>) {
                let read = self.0.read().unwrap();
                match *read {
                    $(
                        $name::$element((ref x, idx, ref parent)) => {
                            debug!("Visiting concept {}: {}", stringify!($element), x.get_uuid().unwrap());
                            for child in x.get_descendants() {
                                child.populate_child_concept_map(concept_map);
                            }
                            concept_map.insert(
                                (
                                    x.get_uuid().unwrap(),
                                    stringify!($element).to_string()
                                 ),
                                 AoristRef(Arc::new(RwLock::new(
                                    $name::$element((x.clone(), idx, parent.clone()))
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
            fn compute_args(
                &self,
                root: <Self::TAncestry as Ancestry>::TConcept,
                ancestry: &Self::TAncestry,
                context: &mut aorist_primitives::Context,
            ) -> (String, String, ParameterTuple, Dialect) {
                let gil = Python::acquire_gil();
                let py = gil.python();
                //let mut args = Vec::new();
                let dill: &PyModule = PyModule::import(py, "dill").unwrap();
                let mut args = Vec::new();
                let mut kwargs = LinkedHashMap::new();
                for (input_types, serialized) in &self.inner.get_arg_functions() {
                    let py_arg = PyString::new(py, &serialized);
                    let deserialized = dill.getattr("loads").unwrap().call1((py_arg,)).unwrap();
                    let mut objects = Vec::new();
                    let mut context_pos = None;
                    for (i, x) in input_types.iter().enumerate() {
                        if x == "context" {
                            assert!(context_pos.is_none());
                            context_pos = Some(i);
                        } else {
                            objects.push(
                                ancestry.py_object(x, root.clone(), py).unwrap().to_object(py)
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
                        context.insert(&extracted_context);
                        extracted = extracted_string;
                    } else {
                        let arg = deserialized.call1((objects,)).unwrap();
                        // TODO: add more return types here
                        extracted = arg.extract().unwrap();
                    }
                    let ast = AST::StringLiteral(StringLiteral::new_wrapped(extracted, false));
                    args.push(ast);
                }
                for (key, (input_types, serialized)) in &self.inner.get_kwarg_functions() {
                    let py_arg = PyString::new(py, &serialized);
                    let py_arg = py_arg.call_method1("encode", ("latin-1",)).unwrap();
                    let deserialized = dill.getattr("loads").unwrap().call1((py_arg,)).unwrap();


                    let mut objects = Vec::new();
                    let mut context_pos = None;
                    for (i, x) in input_types.iter().enumerate() {
                        if x == "context" {
                            assert!(context_pos.is_none());
                            context_pos = Some(i);
                        } else {
                            match ancestry.py_object(x, root.clone(), py) {
                                Ok(x) => objects.push(x.to_object(py)),
                                Err(err) => panic!(
                                    "Error when running program for key {} input_type {} # {}:\n{}",
                                    key, i, x, err,
                                ),
                            }
                        }
                    }
                    if let Some(pos) = context_pos {
                        let obj = PyObject::from(PyCell::new(py, context.clone()).unwrap());
                        objects.insert(pos, obj.to_object(py));
                    };
                    let extracted: AST = match deserialized.call1((objects,)) {
                        Ok(arg) => {
                            let result = match context_pos {
                                Some(_) => aorist_ast::extract_arg_with_context(arg, context),
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
                            panic!("Problem when extracting object. See traceback above");
                        }
                    };

                    if key.as_bytes()[0] != '_' as u8 {
                        kwargs.insert(key.to_string(), extracted);
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
            pub fn get_arg_functions(&self) -> Vec<(Vec<String>, String)> {
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
            pub fn get_code(&self) -> String {
                match self {
                    $(
                        [<$name ProgramEnum>]::$element(x) => x.get_code(),
                    )+
                }
            }
            pub fn get_entrypoint(&self) -> String {
                match self {
                    $(
                        [<$name ProgramEnum>]::$element(x) => x.get_entrypoint(),
                    )+
                }
            }
            pub fn get_kwarg_functions(&self) -> LinkedHashMap<String, (Vec<String>, String)> {
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
            fn builders() -> Vec<[<$name Builder>]<$lt>> where Self : Sized {
                vec![
                    $(
                        [<$name Builder>]::$element(
                            ConstraintBuilder::<$lt, $element>{
                                _phantom: std::marker::PhantomData,
                                _phantom_lt: std::marker::PhantomData,
                            }
                        ),
                    )+
                ]
            }
            fn get_constraint_name(&self) -> String {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => stringify!($element).to_string(),
                    )+
                }
            }
            fn get_required_constraint_names(&self) -> Vec<String> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_required_constraint_names(),
                    )+
                }
            }
            fn build_constraint(
                &self,
                root_uuid: Uuid,
                potential_child_constraints: Vec<Arc<RwLock<Self::OuterType>>>,
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
            fn get_root_type_name(&self) -> Result<String> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_root_type_name(),
                    )+
                }
            }
            fn get_required(&self, root: AoristRef<Concept>, ancestry:&ConceptAncestry) -> Vec<Uuid> {
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
            fn get_required_constraint_names() -> HashMap<String, Vec<String>> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_required_constraint_names(),
                    )+
                }
            }
            fn get_explanations() -> HashMap<String, (Option<String>, Option<String>)> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => (
                            $element::get_title(),
                            $element::get_body(),
                        ),
                    )+
                }
            }
        }
        impl <$lt> $name {
            pub fn get_root_type_name(&self) -> Result<String> {
                match self {
                    $(
                        Self::$element(_) => $element::get_root_type_name(),
                    )+
                }
            }
            pub fn get_downstream_constraints(&self) -> Result<Vec<Arc<RwLock<Constraint>>>> {
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
            pub fn get_uuid(&self) -> Result<Uuid> {
                match self {
                    $(
                        Self::$element(x) => x.get_uuid(),
                    )+
                }
            }
            pub fn get_title(&self) -> Option<String> {
                match self {
                    $(
                        Self::$element(_) => $element::get_title(),
                    )+
                }
            }
            pub fn get_body(&self) -> Option<String> {
                match self {
                    $(
                        Self::$element(_) => $element::get_body(),
                    )+
                }
            }
            pub fn get_root_uuid(&self) -> Result<Uuid> {
                match self {
                    $(
                        Self::$element(x) => x.get_root_uuid(),
                    )+
                }
            }
            fn get_root_type_names() -> Result<HashMap<String, String>> {
                Ok(hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_root_type_name()?,
                    )+
                })
            }
            pub fn get_name(&self) -> String {
                match self {
                    $(
                        Self::$element(x) => stringify!($element).to_string(),
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

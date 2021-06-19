#![allow(non_snake_case)]
use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::{BTreeSet, HashMap};
use std::hash::Hasher;
use uuid::Uuid;

#[macro_export]
macro_rules! register_ast_nodes {
    ($name:ident, $($variant: ident,)+) => {

        #[derive(Clone)]
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
        #[derive(Hash, PartialEq, Eq, Clone)]
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
        #[derive(Hash, PartialEq, Eq, Clone, Optimizable)]
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
            fn new($(
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
macro_rules! register_programs_for_constraint {
    ($constraint:ident, $root: ident, $lt: lifetime, $clt: lifetime, $ancestry: ty,
     $($dialect:ident, $element: ident),+) => {
        impl<$lt, $clt> SatisfiableConstraint<$lt, $clt> for $constraint where $lt : $clt {
            type TAncestry = $ancestry;
            fn satisfy(
                &mut self,
                c: Concept<$lt>,
                d: &Dialect,
                ancestry: Arc<$ancestry>,
            ) -> Result<Option<(String, String, ParameterTuple, Dialect)>> {
                match d {
                    $(
                        Dialect::$dialect{..} => Ok(Some((
                            $element::get_preamble(),
                            $element::get_call(),
                            $element::compute_parameter_tuple(
                                self.get_uuid()?.clone(),
                                c.clone(),
                                ancestry,
                            ),
                            $element::get_dialect(),
                        ))),
                    )+
                    _ => Ok(None),
                }
            }
            fn satisfy_given_preference_ordering(
                &mut self,
                c: Concept<$lt>,
                preferences: &Vec<Dialect>,
                ancestry: Arc<$ancestry>,
            ) -> Result<(String, String, ParameterTuple, Dialect)> {
                match c {
                    Concept::$root{..} => {
                        for d in preferences {
                            if let Some((preamble, call, params, dialect))
                                = self.satisfy(c.clone(), &d, ancestry.clone())? {
                                return Ok((preamble, call, params, dialect));
                            }
                        }
                        bail!("Cannot satisfy preference ordering for {}", c.get_uuid())
                    },
                    _ => bail!("Wrong type of concept provided: {}", c.get_type())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! register_satisfiable_constraints {

    ($outer: ident, $($constraint:ident),+)  => {
        impl <'a> SatisfiableOuterConstraint<'a> for $outer {
            fn satisfy_given_preference_ordering(
                &mut self,
                c: Concept<'a>,
                preferences: &Vec<Dialect>,
                ancestry: Arc<ConceptAncestry<'a>>,
            ) -> Result<(String, String, ParameterTuple, Dialect)> {
                match &mut self.inner {
                    $(
                        Some(AoristConstraint::$constraint(ref mut x)) =>
                        x.satisfy_given_preference_ordering(
                            c, preferences,
                            ancestry,
                        ),
                    )+
                    _ => bail!("Constraint {} is not satisfiable (no program provided).", self.inner.as_ref().unwrap().get_name())
                }
            }
        }
    }
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
        paste::item! {
            #[cfg_attr(feature = "python", pyclass)]
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
            #[cfg_attr(feature = "python", pymethods)]
            #[cfg(feature = "python")]
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
        }
    };
}

#[macro_export]
macro_rules! define_constraint {
    ($element:ident, $requires_program:expr, $satisfy_type:ident, $root:ident, $outer:ident,
    $title:expr, $body:expr, $should_add:expr, $get_required:expr $(, $required:ident)*) => {
        paste::item! {
            pub struct $element {
                id: Uuid,
                root_uuid: Uuid,
                $([<$required:snake:lower>] : Vec<Arc<RwLock<$outer>>>,)*
            }
            pub trait $satisfy_type<'a, 'b> : ConstraintSatisfactionBase<'a, 'b, ConstraintType=$element, RootType=$root> where 'a : 'b {
                type Dialect;

                // computes a parameter tuple as a string, e.g. to be called from
                // Python
                fn compute_parameter_tuple(
                    uuid: Uuid,
                    root: Concept<'a>,
                    ancestry: Arc<ConceptAncestry<'a>>,
                ) -> ParameterTuple;
                fn get_preamble() -> String;
                fn get_call() -> String;
                fn get_dialect() -> Dialect;
            }
            impl $element {
                pub fn get_uuid(&self) -> Result<Uuid> {
                    Ok(self.id.clone())
                }
                pub fn _should_add<'a>(root: Concept<'a>, ancestry: &ConceptAncestry<'a>) -> bool {
                    $should_add(root, ancestry)
                }
                pub fn get_required<'a>(root: Concept<'a>, ancestry: &ConceptAncestry<'a>) -> Vec<Uuid> {
                    $get_required(root, ancestry)
                }
                pub fn get_root_uuid(&self) -> Result<Uuid> {
                    Ok(self.root_uuid.clone())
                }
                pub fn requires_program(&self) -> Result<bool> {
                    Ok($requires_program)
                }
                // these are *all* downstream constraints
                pub fn get_downstream_constraints(&self) -> Result<Vec<Arc<RwLock<Constraint>>>> {
                    let mut downstream: Vec<Arc<RwLock<Constraint>>> = Vec::new();
                    $(
                        for arc in &self.[<$required:snake:lower>] {
                            downstream.push(arc.clone());
                        }
                    )*
                    Ok(downstream)
                }
                pub fn get_title() -> Option<String> {
                    $title
                }
                pub fn get_body() -> Option<String> {
                    $body
                }
            }
            impl <'a, 'b> TConstraint<'a, 'b> for $element where 'a : 'b {
                type Root = $root;
                type Outer = $outer;
                type Ancestry = ConceptAncestry<'a>;

                fn get_root_type_name() -> Result<String> {
                    Ok(stringify!($root).into())
                }
                fn get_required_constraint_names() -> Vec<String> {
                    vec![$(
                        stringify!($required).into()
                    ),*]
                }
                fn should_add(root: Concept<'a>, ancestry: &ConceptAncestry<'a>) -> bool {
                    match &root {
                        Concept::$root(x) => Self::_should_add(root, ancestry),
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
macro_rules! register_constraint {
    ( $name:ident, $lt: lifetime, $clt: lifetime, $($element: ident),+ ) => { paste::item! {
        pub enum $name {
            $(
                $element($element),
            )+
        }
        pub enum [<$name Builder>]<$lt, $clt> where $lt: $clt {
            $(
                $element(crate::constraint::ConstraintBuilder<$lt, $clt, $element>),
            )+
        }
        impl <$lt, $clt> [<$name Builder>]<$lt, $clt>
        where $lt : $clt {
            pub fn get_root_type_name(&self) -> Result<String> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_root_type_name(),
                    )+
                }
            }
            pub fn get_required(&$clt self, root: Concept<$lt>, ancestry:&ConceptAncestry<$lt>) -> Vec<Uuid> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) =>
                        $element::get_required(root, ancestry),
                    )+
                }
            }
            pub fn should_add(&$clt self, root: Concept<$lt>, ancestry:&ConceptAncestry<$lt>) -> bool {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) =>
                        $element::should_add(root, ancestry),
                    )+
                }
            }
            pub fn build_constraint(
                &self,
                root_uuid: Uuid,
                potential_child_constraints: Vec<Arc<RwLock<Constraint>>>,
            ) -> Result<Constraint> {
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
            pub fn get_required_constraint_names(&self) -> Vec<String> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_required_constraint_names(),
                    )+
                }
            }
            pub fn get_constraint_name(&self) -> String {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => stringify!($element).to_string(),
                    )+
                }
            }
        }
        impl <$lt, $clt> $name where $lt : $clt {
            pub fn builders() -> Vec<[<$name Builder>]<$lt, $clt>> {
                vec![
                    $(
                        [<$name Builder>]::$element(
                            crate::constraint::ConstraintBuilder::<$lt, $clt, $element>{
                                _phantom: std::marker::PhantomData,
                                _phantom_lt: std::marker::PhantomData,
                                _phantom_clt: std::marker::PhantomData
                            }
                        ),
                    )+
                ]
            }
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
            pub fn get_required_constraint_names() -> HashMap<String, Vec<String>> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_required_constraint_names(),
                    )+
                }
            }
            pub fn get_explanations() -> HashMap<String, (Option<String>,
            Option<String>)> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => (
                            $element::get_title(),
                            $element::get_body(),
                        ),
                    )+
                }
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
                root: Concept<$lt>,
                ancestry: &ConceptAncestry<$lt>
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
        paste::item!(
            pub fn [<$name:snake:lower>] (m: &PyModule) -> PyResult<()> {
                $(
                    m.add_class::<$element>()?;
                )+
                Ok(())
            }
        );
    }}
}

#[macro_export]
macro_rules! register_attribute {
    ( $name:ident, $($element: ident),+ ) => { paste! {
        #[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
        pub enum [<$name Enum>] {
            $(
                $element($element),
            )+
        }
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
            pub fn get_sql_type(&self) -> DataType {
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
        #[aorist_concept(derivative(Hash))]
        pub struct $name {
            pub inner: AttributeOrTransform,
        }
        impl<'a> FromPyObject<'a> for $name {
            fn extract(ob: &'a PyAny) -> PyResult<Self> {
                let inner = AttributeOrTransform::extract(ob)?;
                Ok(Self{ inner, constraints: Vec::new(), tag: None, uuid: None })
            }
        }
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
        paste::item!(
            pub fn [<$name:snake:lower>] (m: &PyModule) -> PyResult<()> {
                $(
                    m.add_class::<$element>()?;
                )+
                Ok(())
            }
        );
    }}
}

#[macro_export]
macro_rules! register_concept {
    ( $name:ident, $ancestry:ident, $($element: ident ),* ) => { paste::item! {
        #[derive(Clone)]
        pub enum $name<'a> {
            $(
                $element((&'a $element, usize, Option<(Uuid, String)>)),
            )+
        }
        $(
            impl <'a> [<CanBe $element>]<'a> for $name<'a> {
                fn [<construct_ $element:snake:lower>](
                    obj_ref: &'a $element,
                    ix: Option<usize>,
                    id: Option<(Uuid, String)>
                ) -> Self {
                    $name::$element((
                        obj_ref,
                        match ix {
                            Some(i) => i,
                            None => 0,
                        },
                        id,
                    ))
               }
            }
        )+
        $(
            impl $element {
                fn get_descendants<'a>(&'a self) -> Vec<$name<'a>> {
                    let mut concepts = Vec::new();
                    for tpl in self.get_children() {
                        let wrapped_concept = WrappedConcept::from(tpl);
                        concepts.push(wrapped_concept.inner);
                    }
                    concepts
                }
            }
        )+

        impl <'a> ConceptEnum<'a> for $name<'a> {}
        $(
            impl <'a> TryFrom<$name<'a>> for &'a $element {
                type Error = String;
                fn try_from(x: $name<'a>) -> Result<Self, String> {
                    match x {
                        $name::$element((y, _, _)) => Ok(y),
                        _ => Err("Cannot convert.".into()),
                    }
                }
            }
            impl <'a> TryFrom<&'a $name<'a>> for &'a $element {
                type Error = String;
                fn try_from(x: &'a $name<'a>) -> Result<Self, String> {
                    match x {
                        &$name::$element((y, _, _)) => Ok(y),
                        _ => Err("Cannot convert.".into()),
                    }
                }
            }
        )+
        pub struct $ancestry<'a> {
            pub parents: Arc<RwLock<HashMap<(Uuid, String), $name<'a>>>>,
        }
        impl <'a> Ancestry<'a> for $ancestry<'a> {
            type TConcept = $name<'a>;
        }
        impl <'a> $ancestry<'a> {
            $(
                pub fn [<$element:snake:lower>](
                    &self,
                    root: $name<'a>,
                ) -> Result<&'a $element, String> {
                    if root.get_type() == stringify!($element).to_string(){
                        return(Ok(<&'a $element>::try_from(root).unwrap()));
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
        impl <'a> $name<'a> {
            pub fn get_parent_id(&'a self) -> Option<(Uuid, String)> {
                match self {
                    $(
                        $name::$element((_, _, id)) => id.clone(),
                    )+
                }
            }
            pub fn get_type(&'a self) -> String {
                match self {
                    $(
                        $name::$element((x, _, _)) => stringify!($element).to_string(),
                    )*
                }
            }
            pub fn get_uuid(&'a self) -> Uuid {
                match self {
                    $(
                        $name::$element((x, _, _)) => x.get_uuid(),
                    )*
                }
            }
            pub fn get_tag(&'a self) -> Option<String> {
                match self {
                    $(
                        $name::$element((x, _, _)) => x.get_tag(),
                    )*
                }
            }
            pub fn get_index_as_child(&'a self) -> usize {
                match self {
                    $(
                        $name::$element((_, idx, _)) => *idx,
                    )*
                }
            }
            pub fn get_child_concepts<'b>(&'a self) -> Vec<$name<'b>> where 'a : 'b {
                match self {
                    $(
                        $name::$element((x, _, _)) => x.get_descendants(),
                    )*
                }
            }
            pub fn populate_child_concept_map(&self, concept_map: &mut HashMap<(Uuid, String), Concept<'a>>) {
                match self {
                    $(
                        $name::$element((ref x, idx, parent)) => {
                            debug!("Visiting concept {}: {}", stringify!($element), x.get_uuid());
                            for child in x.get_descendants() {
                                child.populate_child_concept_map(concept_map);
                            }
                            concept_map.insert(
                                (x.get_uuid(),
                                 stringify!($element).to_string()),
                                 $name::$element((&x, *idx, parent.clone())),
                            );
                        }
                    )*
                }
            }
        }
    }
    }
}
pub trait ConceptEnum<'a> {}
pub trait AoristConcept<'a> {
    type TChildrenEnum: ConceptEnum<'a>;

    fn get_children(
        &'a self,
    ) -> Vec<(
        // struct name
        &str,
        // field name
        Option<&str>,
        // ix
        Option<usize>,
        // uuid
        Option<Uuid>,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
    fn get_uuid(&self) -> Uuid;
    fn get_children_uuid(&self) -> Vec<Uuid>;
    fn get_tag(&self) -> Option<String>;

    fn get_uuid_from_children_uuid(&self) -> Uuid {
        let child_uuids = self.get_children_uuid();
        if child_uuids.len() > 0 {
            eprintln!("There are child uuids.");
            let uuids = child_uuids.into_iter().collect::<BTreeSet<Uuid>>();
            let mut hasher = SipHasher::new();
            for uuid in uuids {
                hasher.write(uuid.as_bytes());
            }
            let bytes: [u8; 16] = hasher.finish128().as_bytes();
            Uuid::from_bytes(bytes)
        } else {
            eprintln!("There are no child uuids.");
            // TODO: this should just be created from the hash
            Uuid::new_v4()
        }
    }
    fn compute_uuids(&mut self);
}
pub trait TConceptEnum<'a>: Sized {
    fn get_parent_id(&'a self) -> Option<(Uuid, String)>;
    fn get_type(&'a self) -> String;
    fn get_uuid(&'a self) -> Uuid;
    fn get_tag(&'a self) -> Option<String>;
    fn get_index_as_child(&'a self) -> usize;
    fn get_child_concepts<'b, T: TConceptEnum<'b>>(&'a self) -> Vec<T> where 'a : 'b;
    fn populate_child_concept_map(&self, concept_map: &mut HashMap<(Uuid, String), Self>);
}
#[macro_export]
macro_rules! register_constraint_new {
    ( $name:ident, $lt: lifetime, $clt: lifetime, $($element: ident),+ ) => { paste::item! {
        pub enum $name {
            $(
                $element($element),
            )+
        }
        pub enum [<$name Builder>]<$lt, $clt> where $lt: $clt {
            $(
                $element(ConstraintBuilder<$lt, $clt, $element>),
            )+
        }
        impl <$lt, $clt> [<$name Builder>]<$lt, $clt>
        where $lt : $clt {
            pub fn get_root_type_name(&self) -> Result<String> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_root_type_name(),
                    )+
                }
            }
            pub fn get_required(&$clt self, root: Concept<$lt>, ancestry:&ConceptAncestry<$lt>) -> Vec<Uuid> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) =>
                        $element::get_required(root, ancestry),
                    )+
                }
            }
            pub fn should_add(&$clt self, root: Concept<$lt>, ancestry:&ConceptAncestry<$lt>) -> bool {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) =>
                        $element::should_add(root, ancestry),
                    )+
                }
            }
            pub fn build_constraint(
                &self,
                root_uuid: Uuid,
                potential_child_constraints: Vec<Arc<RwLock<Constraint>>>,
            ) -> Result<Constraint> {
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
            pub fn get_required_constraint_names(&self) -> Vec<String> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_required_constraint_names(),
                    )+
                }
            }
            pub fn get_constraint_name(&self) -> String {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => stringify!($element).to_string(),
                    )+
                }
            }
        }
        impl <$lt, $clt> $name where $lt : $clt {
            pub fn builders() -> Vec<[<$name Builder>]<$lt, $clt>> {
                vec![
                    $(
                        [<$name Builder>]::$element(
                            ConstraintBuilder::<$lt, $clt, $element>{
                                _phantom: std::marker::PhantomData,
                                _phantom_lt: std::marker::PhantomData,
                                _phantom_clt: std::marker::PhantomData
                            }
                        ),
                    )+
                ]
            }
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
            pub fn get_required_constraint_names() -> HashMap<String, Vec<String>> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_required_constraint_names(),
                    )+
                }
            }
            pub fn get_explanations() -> HashMap<String, (Option<String>,
            Option<String>)> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => (
                            $element::get_title(),
                            $element::get_body(),
                        ),
                    )+
                }
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
                root: Concept<$lt>,
                ancestry: &ConceptAncestry<$lt>
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

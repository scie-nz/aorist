#![allow(non_snake_case)]
use indoc::formatdoc;
use sqlparser::ast::{ColumnDef, DataType, Ident};


#[macro_export]
macro_rules! define_program {
    ($name:ident, $root:ident, $constraint:ident, $satisfy_type:ident,
     $dialect:ident,
     $preamble:expr, $call:expr, $tuple_call: expr) => {
        pub struct $name {
        }
        impl ConstraintSatisfactionBase for $name {
            type RootType = $root;
            type ConstraintType = $constraint;
        }
        impl $satisfy_type for $name {
            type Dialect = $dialect;
            fn compute_parameter_tuple(
                root: &$root
            ) -> String {
                $tuple_call(root)
            }
            fn get_preamble() -> String {
                $preamble.to_string()
            }
            fn get_call() -> String {
                $call.to_string()
            }
        }
    };
}

#[macro_export]
macro_rules! register_programs_for_constraint {
    ($name:ident, $constraint:ident,
     $($dialect:ident, $element: ident),+) => {

        pub trait $name: TConstraint {
            fn satisfy(&self, r: &<Self as TConstraint>::Root, d: Dialect) -> Option<(String, String, String)>;
        }
        impl $name for $constraint {
            fn satisfy(&self, r: &<Self as TConstraint>::Root, d: Dialect) -> Option<(String, String, String)> {
                match d {
                    $(
                        Dialect::$dialect{..} => Some((
                            $element::get_preamble(),
                            $element::get_call(),
                            $element::compute_parameter_tuple(r),
                        )),
                    )+
                    _ => None,
                }
            }
        }

    };
}

#[macro_export]
macro_rules! define_attribute {
    ($element:ident, $presto_type:ident, $orc_type:ident, $sql_type:ident) => {
        #[derive(
            Derivative,
            Serialize,
            Deserialize,
            Clone,
            Constrainable,
            $presto_type,
            $orc_type,
            $sql_type,
        )]
        #[derivative(PartialEq, Debug)]
        pub struct $element {
            name: String,
            comment: Option<String>,
            uuid: Option<Uuid>,
            #[serde(skip)]
            #[derivative(PartialEq = "ignore", Debug = "ignore")]
            constraints: Vec<Arc<RwLock<Constraint>>>,
        }
        impl TAttribute for $element {
            fn get_name(&self) -> &String {
                &self.name
            }
            fn get_comment(&self) -> &Option<String> {
                &self.comment
            }
        }
    };
}

#[macro_export]
macro_rules! define_constraint {
    ($element:ident, $requires_program:expr, $satisfy_type:ident, $root:ident) => {
        pub struct $element {
            id: Uuid,
            root_uuid: Uuid,
        }
        impl $element {
            pub fn new(root_uuid: Uuid,
                       _potential_child_constraints: Vec<Arc<RwLock<Constraint>>>) -> Self {
                Self{ id: Uuid::new_v4(), root_uuid }
            }
            pub fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
                Vec::new()
            }
            pub fn get_uuid(&self) -> Uuid {
                self.id.clone()
            }
            pub fn get_root_uuid(&self) -> Uuid {
                self.root_uuid.clone()
            }
            pub fn requires_program(&self) -> bool {
                $requires_program
            }
            pub fn ingest_upstream_constraints(
                &self,
                _upstream_constraints: Vec<Arc<RwLock<Constraint>>>
            ) {}
        }
        pub trait $satisfy_type: ConstraintSatisfactionBase<ConstraintType=$element, RootType=$root> {
            type Dialect;

            // computes a parameter tuple as a string, e.g. to be called from
            // Python
            fn compute_parameter_tuple(root: &<Self as ConstraintSatisfactionBase>::RootType) -> String;
            fn get_preamble() -> String;
            fn get_call() -> String;
        }
        impl TConstraint for $element {
            type Root = $root;

            fn get_root_type_name() -> String {
                stringify!($root).into()
            }
            fn get_required_constraint_names() -> Vec<String> {
                Vec::new()
            }
        }
		impl fmt::Debug for $element {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.debug_struct(stringify!($element))
				 .field("id", &self.id)
				 .finish()
			}
		}
    };
    ($element:ident, $requires_program:expr, $satisfy_type:ident, $root:ident, $($required:ident),+) => {
        paste::item! {
            pub struct $element {
                id: Uuid,
                root_uuid: Uuid,
                $([<$required:snake:lower>] : Vec<Arc<RwLock<Constraint>>>,)+
            }
            impl $element {
                pub fn get_uuid(&self) -> Uuid {
                    self.id.clone()
                }
                pub fn get_root_uuid(&self) -> Uuid {
                    self.root_uuid.clone()
                }
                pub fn requires_program(&self) -> bool {
                    $requires_program
                }
                pub fn ingest_upstream_constraints(
                    &mut self,
                    upstream_constraints: Vec<Arc<RwLock<Constraint>>>
                ) {
                    for constraint in upstream_constraints {
                        $(
                            if let Some(AoristConstraint::$required(x)) =
                            &constraint.read().unwrap().inner
                            {
                                self.[<$required:snake:lower>].push(constraint.clone());
                            }
                        )+
                    }
                }
                // these are *all* downstream constraints
                pub fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
                    let mut downstream: Vec<Arc<RwLock<Constraint>>> = Vec::new();
                    $(
                        for arc in &self.[<$required:snake:lower>] {
                            downstream.push(arc.clone());
                        }
                    )+
                    downstream
                }
                pub fn new(root_uuid: Uuid,
                           potential_child_constraints: Vec<Arc<RwLock<Constraint>>>) -> Self {
                    // TODO: we should dedupe potential child constraints
                    $(
                        let mut [<$required:snake:lower>]: Vec<Arc<RwLock<Constraint>>> =
                        Vec::new();
                    )+
                    let mut actual_child_constraints: Vec<Arc<RwLock<Constraint>>> = Vec::new();

                    for constraint in &potential_child_constraints {
                        $(
                            if let Some(AoristConstraint::$required{..}) =
                            &constraint.read().unwrap().inner
                            {
                                actual_child_constraints.push(constraint.clone());
                            }
                        )+
                    }
                    let by_uuid: HashMap<Uuid, Arc<RwLock<Constraint>>> =
                    actual_child_constraints
                        .into_iter().map(|x| (x.clone().read().unwrap().get_uuid(), x)).collect();
                    for constraint in by_uuid.values() {
                        $(
                            if let Some(AoristConstraint::$required{..}) =
                            &constraint.read().unwrap().inner {
                                [<$required:snake:lower>].push(constraint.clone());
                            }
                        )+
                    }
                    Self{
                        id: Uuid::new_v4(),
                        root_uuid,
                        $([<$required:snake:lower>],)+
                    }
                }
            }
            pub trait $satisfy_type: ConstraintSatisfactionBase<ConstraintType=$element, RootType=$root> {
                fn satisfy(
                    root: &<Self as ConstraintSatisfactionBase>::RootType,
                    constraint: &<Self as ConstraintSatisfactionBase>::ConstraintType,
                ) -> String;
            }
            impl TConstraint for $element {
                type Root = $root;
                fn get_root_type_name() -> String {
                    stringify!($root).into()
                }
                fn get_required_constraint_names() -> Vec<String> {
                    vec![$(
                        stringify!($required).into()
                    ),+]
                }
            }
        }
    };
}
#[macro_export]
macro_rules! register_constraint {
    ( $name:ident, $($element: ident),+ ) => {
        pub enum $name {
            $(
                $element($element),
            )+
        }
        impl $name {
            pub fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
                match self {
                    $(
                        Self::$element(x) => x.get_downstream_constraints(),
                    )+
                }
            }
            pub fn requires_program(&self) -> bool {
                match self {
                    $(
                        Self::$element(x) => x.requires_program(),
                    )+
                }
            }
            pub fn ingest_upstream_constraints(
                &mut self,
                upstream_constraints: Vec<Arc<RwLock<Constraint>>>
            ) {
                match self {
                    $(
                        Self::$element(ref mut x) =>
                        x.ingest_upstream_constraints(upstream_constraints),
                    )+
                }
            }
            pub fn get_uuid(&self) -> Uuid {
                match self {
                    $(
                        Self::$element(x) => x.get_uuid(),
                    )+
                }
            }
            pub fn get_root_uuid(&self) -> Uuid {
                match self {
                    $(
                        Self::$element(x) => x.get_root_uuid(),
                    )+
                }
            }
            fn get_root_type_names() -> HashMap<String, String> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_root_type_name(),
                    )+
                }
            }
            fn get_required_constraint_names() -> HashMap<String, Vec<String>> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_required_constraint_names(),
                    )+
                }
            }
        }
    }
}
#[macro_export]
macro_rules! register_attribute {
    ( $name:ident, $($element: ident),+ ) => {
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
        #[serde(tag = "type")]
        pub enum $name {
            $(
                $element($element),
            )+
        }
        impl TAttribute for $name {
            fn get_name(&self) -> &String {
                match self {
                    $(
                        $name::$element(x) => x.get_name(),
                    )+
                }
            }
            fn get_comment(&self) -> &Option<String> {
                match self {
                    $(
                        $name::$element(x) => x.get_comment(),
                    )+
                }
            }
        }
        impl TPrestoAttribute for $name {
            fn get_presto_type(&self) -> String {
                match self {
                    $(
                        $name::$element(x) => x.get_presto_type(),
                    )+
                }
            }
        }
        impl TOrcAttribute for $name {
            fn get_orc_type(&self) -> String {
                match self {
                    $(
                        $name::$element(x) => x.get_orc_type(),
                    )+
                }
            }
        }
        impl TSQLAttribute for $name {
            fn get_sql_type(&self) -> DataType {
                match self {
                    $(
                        $name::$element(x) => x.get_sql_type(),
                    )+
                }
            }
        }
    }
}

pub trait TAttribute {
    fn get_name(&self) -> &String;
    fn get_comment(&self) -> &Option<String>;
}
pub trait TPrestoAttribute: TAttribute {
    fn get_presto_type(&self) -> String;
    fn get_presto_schema(&self, max_attribute_length: usize) -> String {
        let name = self.get_name();
        let num_middle_spaces = (max_attribute_length - name.len()) + 1;
        let spaces = (0..num_middle_spaces).map(|_| " ").collect::<String>();
        let first_line = format!("{}{}{}", self.get_name(), spaces, self.get_presto_type(),);
        if let Some(comment) = self.get_comment() {
            let formatted_with_comment = formatdoc!(
                "
                {first_line}
                     COMMENT '{comment}'",
                first_line = first_line,
                comment = comment.trim().replace("'", "\\'").to_string()
            );
            return formatted_with_comment;
        }
        first_line
    }
}
pub trait TOrcAttribute: TAttribute {
    fn get_orc_type(&self) -> String;
    fn get_orc_schema(&self) -> String {
        format!("{}:{}", self.get_name(), self.get_orc_type()).to_string()
    }
}
pub trait TSQLAttribute: TAttribute {
    fn get_sql_type(&self) -> DataType;
    fn get_coldef(&self) -> ColumnDef {
        ColumnDef {
            name: Ident::new(self.get_name()),
            data_type: self.get_sql_type(),
            collation: None,
            // TODO: add comments here
            options: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Python {}
#[derive(Debug)]
pub struct R {}
#[derive(Debug)]
pub struct Bash {}

#[derive(Debug)]
pub enum Dialect {
    Python(Python),
    R(R),
    Bash(Bash),
}

pub trait DownloadDataFromRemote {
    // TODO: change this to proper error
    fn get_call(&self, dialect: Dialect) -> Result<String, String>;
}

#[macro_export]
macro_rules! register_concept {
    ( $name:ident, $($element: ident),+ ) => {
        #[derive(Clone)]
        pub enum $name<'a> {
            $(
                $element(&'a $element),
            )+
        }
        impl <'a> $name<'a> {
            pub fn get_uuid(&'a self) -> Uuid {
                match self {
                    $(
                        $name::$element(x) => x.get_uuid(),
                    )*
                }
            }
            pub fn get_child_concepts<'b>(&'a self) -> Vec<$name<'b>> where 'a : 'b {
                match self {
                    $(
                        $name::$element(x) => x.get_child_concepts(),
                    )*
                }
            }
            pub fn populate_child_concept_map(&self, concept_map: &mut HashMap<Uuid, Concept<'a>>) {
                match self {
                    $(
                        $name::$element(ref x) => {
                            for child in x.get_child_concepts() {
                                child.populate_child_concept_map(concept_map);
                            }
                            concept_map.insert(x.get_uuid(), $name::$element(&x));
                        }
                    )*
                }
            }
        }
    }
}

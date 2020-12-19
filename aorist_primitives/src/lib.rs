#![allow(non_snake_case)]
use indoc::formatdoc;
use sqlparser::ast::{ColumnDef, DataType, Ident};

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
    ($element:ident, $root:ident) => {
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
    ($element:ident, $root:ident, $($required:ident),+) => {
        paste::item! {
            pub struct $element {
                id: Uuid,
                root_uuid: Uuid,
                $([<$required:snake:lower>] : Vec<Arc<RwLock<Constraint>>>,)+
                downstream_constraints: Vec<Arc<RwLock<Constraint>>>,
            }
            impl $element {
                pub fn get_uuid(&self) -> Uuid {
                    self.id.clone()
                }
                pub fn get_root_uuid(&self) -> Uuid {
                    self.root_uuid.clone()
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
                    self.downstream_constraints.clone()
                }
                pub fn new(root_uuid: Uuid,
                           potential_child_constraints: Vec<Arc<RwLock<Constraint>>>) -> Self {
                    // TODO: we should dedupe potential child constraints
                    $(
                        let mut [<$required:snake:lower>]: Vec<Arc<RwLock<Constraint>>> =
                        Vec::new();
                    )+
                    for constraint in &potential_child_constraints {
                        $(
                            if let Some(AoristConstraint::$required{..}) =
                            &constraint.read().unwrap().inner
                            {
                                [<$required:snake:lower>].push(constraint.clone());
                            }
                        )+
                    }
                    let by_uuid: HashMap<Uuid, Arc<RwLock<Constraint>>> = potential_child_constraints
                        .into_iter().map(|x| (x.clone().read().unwrap().get_uuid(), x)).collect();
                    Self{
                        id: Uuid::new_v4(),
                        root_uuid,
                        $([<$required:snake:lower>],)+
                        downstream_constraints: by_uuid.into_iter().map(|(_,
                        v)| v).collect(),
                    }
                }
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

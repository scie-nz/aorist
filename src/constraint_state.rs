use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::{AllConstraintsSatisfiability, Constraint};
use crate::object::TAoristObject;
use aorist_primitives::Dialect;
use inflector::cases::snakecase::to_snake_case;
use rustpython_parser::ast::{
    Expression, ExpressionType, Located, Location, StatementType, StringGroup,
};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct ConstraintState<'a> {
    dialect: Option<Dialect>,
    pub key: Option<String>,
    name: String,
    pub satisfied: bool,
    pub satisfied_dependencies: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    pub unsatisfied_dependencies: HashSet<(Uuid, String)>,
    constraint: Arc<RwLock<Constraint>>,
    root: Concept<'a>,
    // these are concept ancestors
    // TODO: change this to Vec<Concept<'a>>
    ancestors: Vec<(Uuid, String, Option<String>, usize)>,
    preamble: Option<String>,
    call: Option<String>,
    params: Option<String>,
    task_name: Option<String>,
}
impl<'a> ConstraintState<'a> {
    pub fn get_task_val(&self, location: Location) -> Expression {
        let outer = Located {
            location,
            node: ExpressionType::Identifier {
                name: "tasks".to_string(),
            },
        };
        let inner = Located {
            location,
            node: ExpressionType::String {
                value: StringGroup::Constant {
                    value: self.get_task_name(),
                },
            },
        };
        Located {
            location,
            node: ExpressionType::Subscript {
                a: Box::new(outer),
                b: Box::new(inner),
            },
        }
    }
    pub fn get_task_statement(&self, location: Location) -> StatementType {
        let val = self.get_task_val(location);
        let _deps = self
            .satisfied_dependencies
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                x.get_task_val(location)
            })
            .collect::<Vec<Expression>>();
        let function = Located {
            location,
            node: ExpressionType::Identifier {
                name: self.get_call().unwrap(),
            },
        };
        // TODO: this is super-hacky, params should be a string enum
        let params = self.params.as_ref().unwrap().clone().replace("\"", "");
        let splits = params.split(", ").collect::<Vec<_>>();
        let args = splits
            .iter()
            .map(|x| {
                Located {
                    location,
                    // TODO: this is where other kinds of arguments can live
                    node: ExpressionType::String {
                        value: StringGroup::Constant {
                            value: x.to_string(),
                        },
                    },
                }
            })
            .collect::<Vec<_>>();
        let task_expr = Located {
            location,
            node: ExpressionType::Call {
                function: Box::new(function),
                args: args,
                keywords: Vec::new(),
                // TODO: add keywords
            },
        };
        let assign = StatementType::Assign {
            targets: vec![val],
            value: task_expr,
        };
        assign
    }
    pub fn set_task_name(&mut self, name: String) {
        self.task_name = Some(name)
    }
    pub fn get_task_name(&self) -> String {
        self.task_name.as_ref().unwrap().clone()
    }
    pub fn get_satisfied_dependency_keys(&self) -> Vec<String> {
        self.satisfied_dependencies
            .iter()
            .map(|x| x.read().unwrap().get_task_name())
            .collect()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    #[allow(dead_code)]
    pub fn get_root(&self) -> Concept<'a> {
        self.root.clone()
    }
    #[allow(dead_code)]
    pub fn get_root_uuid(&self) -> Uuid {
        self.root.get_uuid().clone()
    }
    pub fn get_root_type(&self) -> String {
        self.root.get_type()
    }
    pub fn get_ancestors(&self) -> Vec<(Uuid, String, Option<String>, usize)> {
        self.ancestors.clone()
    }
    pub fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    pub fn get_params(&self) -> Option<String> {
        self.params.clone()
    }
    pub fn get_call(&self) -> Option<String> {
        self.call.clone()
    }
    pub fn get_key(&self) -> Option<String> {
        self.key.clone()
    }
    pub fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    pub fn satisfy(&mut self, preferences: &Vec<Dialect>, ancestry: Arc<ConceptAncestry<'a>>) {
        let root_clone = self.root.clone();
        let constraint = self.constraint.read().unwrap();
        let (preamble, call, params, dialect) = constraint
            .satisfy_given_preference_ordering(root_clone, preferences, ancestry)
            .unwrap();
        self.preamble = Some(preamble);
        self.call = Some(call);
        self.params = Some(params);
        self.dialect = Some(dialect);
    }
    pub fn new(
        constraint: Arc<RwLock<Constraint>>,
        concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
        concept_ancestors: &HashMap<(Uuid, String), Vec<(Uuid, String, Option<String>, usize)>>,
    ) -> Self {
        let arc = constraint.clone();
        let x = arc.read().unwrap();
        let root_uuid = x.get_root_uuid();
        let guard = concepts.read().unwrap();
        let root = guard
            .get(&(root_uuid.clone(), x.root.clone()))
            .unwrap()
            .clone();
        let dependencies = x
            .get_downstream_constraints_ignore_chains()
            .iter()
            .map(|x| (x.read().unwrap().get_uuid(), x.read().unwrap().root.clone()))
            .collect::<HashSet<_>>();
        let ancestors = concept_ancestors
            .get(&(root_uuid, x.root.clone()))
            .unwrap()
            .clone();
        Self {
            dialect: None,
            key: None,
            name: x.get_name().clone(),
            satisfied: false,
            unsatisfied_dependencies: dependencies,
            satisfied_dependencies: Vec::new(),
            constraint,
            root,
            ancestors: ancestors.clone(),
            preamble: None,
            call: None,
            params: None,
            task_name: None,
        }
    }
    pub fn compute_task_name(
        &mut self,
        ancestors: &Vec<(Uuid, String, Option<String>, usize)>,
    ) -> String {
        self.key = Some(match self.root.get_tag() {
            None => {
                let mut relative_path: String = "".to_string();
                for (_, ancestor_type, tag, ix) in ancestors.iter().rev() {
                    if let Some(t) = tag {
                        relative_path = format!("{}__{}", relative_path, t);
                        break;
                    }
                    if *ix > 0 {
                        relative_path = format!(
                            "{}__{}_{}",
                            relative_path,
                            to_snake_case(&ancestor_type),
                            ix
                        );
                    }
                }
                relative_path
            }
            Some(t) => t,
        });
        self.key.as_ref().unwrap().clone()
    }
    pub fn get_fully_qualified_task_name(&self) -> String {
        format!(
            "{}__{}",
            to_snake_case(&self.get_name()),
            self.key.as_ref().unwrap()
        )
    }
}

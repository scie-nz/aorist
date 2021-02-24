use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::{AllConstraintsSatisfiability, Constraint};
use crate::object::TAoristObject;
use crate::python::{Formatted, List, ParameterTuple, SimpleIdentifier, StringLiteral, AST};
use aorist_primitives::Dialect;
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Clone)]
pub struct AncestorRecord {
    pub uuid: Uuid,
    pub object_type: String,
    pub tag: Option<String>,
    pub ix: usize,
}
impl AncestorRecord {
    pub fn new(uuid: Uuid, object_type: String, tag: Option<String>, ix: usize) -> Self {
        Self {
            uuid,
            object_type,
            tag,
            ix,
        }
    }
    pub fn get_key(&self) -> (Uuid, String) {
        (self.uuid.clone(), self.object_type.clone())
    }
}

pub struct ConstraintState<'a> {
    dialect: Option<Dialect>,
    pub key: Option<String>,
    name: String,
    pub satisfied: bool,
    pub satisfied_dependencies: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    pub unsatisfied_dependencies: LinkedHashSet<(Uuid, String)>,
    constraint: Arc<RwLock<Constraint>>,
    root: Concept<'a>,
    // these are concept ancestors
    // TODO: change this to Vec<Concept<'a>>
    ancestors: Vec<AncestorRecord>,
    preamble: Option<String>,
    call: Option<String>,
    params: Option<ParameterTuple>,
    task_name: Option<String>,
    task_val: Option<AST>,
}

impl<'a> ConstraintState<'a> {
    pub fn requires_program(&self) -> bool {
        self.constraint.read().unwrap().requires_program()
    }
    pub fn set_task_val(&mut self, val: AST) {
        self.task_val = Some(val);
    }
    pub fn get_task_val(&self) -> AST {
        self.task_val.as_ref().unwrap().clone()
    }
    pub fn get_dependencies(&self) -> Vec<AST> {
        self.satisfied_dependencies
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                x.get_task_val()
            })
            .collect::<Vec<AST>>()
    }
    pub fn get_dep_list(&self) -> Option<AST> {
        let dep_list = self
            .satisfied_dependencies
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                x.get_task_val()
            })
            .collect::<Vec<AST>>();
        if dep_list.len() == 1 {
            return Some(dep_list.clone().into_iter().next().unwrap());
        } else if dep_list.len() > 1 {
            return Some(AST::List(List::new_wrapped(dep_list, false)));
        } else {
            return None;
        }
    }
    pub fn get_task_call(&self) -> Result<AST, String> {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                self.get_call().unwrap(),
            ))),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => Ok(AST::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("ShellTask".to_string()),
            )),
            None => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "ConstantTask".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
    }
    pub fn get_args_vec(&self) -> Result<Vec<AST>, String> {
        match (&self.params, &self.dialect) {
            (Some(ref p), Some(Dialect::Python(_))) => Ok(p.get_args()),
            (None, Some(Dialect::Python(_))) => Ok(Vec::new()),
            (_, Some(Dialect::Presto(_))) => Ok(Vec::new()),
            (_, Some(Dialect::Bash(_))) => Ok(Vec::new()),
            (_, None) => Ok(vec![AST::StringLiteral(StringLiteral::new_wrapped(
                self.constraint.read().unwrap().get_name().clone(),
                false,
            ))]),
            _ => Err("Dialect not supported".to_string()),
        }
    }
    pub fn get_kwargs_map(&self) -> Result<LinkedHashMap<String, AST>, String> {
        match &self.dialect {
            Some(Dialect::Python(_)) => match self.params {
                Some(ref p) => Ok(p.get_kwargs()),
                None => Ok(LinkedHashMap::new()),
            },
            Some(Dialect::Presto(_)) => {
                let raw_command = format!("presto -e '{}'", self.get_call().unwrap().clone());
                let format_string = StringLiteral::new_wrapped(raw_command.to_string(), true);
                let command = match self.params {
                    Some(ref p) => AST::Formatted(Formatted::new_wrapped(
                        AST::StringLiteral(format_string),
                        p.get_kwargs(),
                    )),
                    None => AST::StringLiteral(format_string),
                };
                let mut keywords: LinkedHashMap<String, AST> = LinkedHashMap::new();
                keywords.insert("command".to_string(), command);
                Ok(keywords)
            }
            Some(Dialect::Bash(_)) => {
                let format_string =
                    AST::StringLiteral(StringLiteral::new_wrapped(self.get_call().unwrap(), false));
                let command = match self.params {
                    Some(ref p) => {
                        AST::Formatted(Formatted::new_wrapped(format_string, p.get_kwargs()))
                    }
                    None => format_string,
                };
                let mut keywords: LinkedHashMap<String, AST> = LinkedHashMap::new();
                keywords.insert("command".to_string(), command);
                Ok(keywords)
            }
            None => Ok(LinkedHashMap::new()),
            _ => Err("Dialect not supported".to_string()),
        }
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
    pub fn get_constraint_uuid(&self) -> Uuid {
        self.constraint.read().unwrap().get_uuid().clone()
    }
    #[allow(dead_code)]
    pub fn get_root_uuid(&self) -> Uuid {
        self.root.get_uuid().clone()
    }
    pub fn get_root_type(&self) -> String {
        self.root.get_type()
    }
    pub fn get_ancestors(&self) -> Vec<AncestorRecord> {
        self.ancestors.clone()
    }
    pub fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    pub fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    pub fn get_params(&self) -> Option<ParameterTuple> {
        self.params.clone()
    }
    pub fn get_call(&self) -> Option<String> {
        self.call.clone()
    }
    pub fn get_key(&self) -> Option<String> {
        self.key.clone()
    }
    pub fn satisfy(&mut self, preferences: &Vec<Dialect>, ancestry: Arc<ConceptAncestry<'a>>) {
        let root_clone = self.root.clone();
        let mut constraint = self.constraint.write().unwrap();
        let (preamble, call, params, dialect) = constraint
            .satisfy_given_preference_ordering(root_clone, preferences, ancestry)
            .unwrap();
        drop(constraint);
        self.preamble = Some(preamble);
        self.call = Some(call);
        self.params = Some(params);
        self.dialect = Some(dialect);
    }
    pub fn new(
        constraint: Arc<RwLock<Constraint>>,
        concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
        concept_ancestors: &HashMap<(Uuid, String), Vec<AncestorRecord>>,
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
            .get_downstream_constraints()
            .iter()
            .map(|x| (x.read().unwrap().get_uuid(), x.read().unwrap().root.clone()))
            .collect::<LinkedHashSet<_>>();

        /*
        println!(
            "Constraint {} on {} {} has the following dependencies",
            x.get_name(),
            x.get_root_type_name(),
            &root_uuid
        );
        for dependency in dependencies.iter() {
            println!("{:?}", dependency);
        }*/
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
            task_val: None,
        }
    }
    pub fn compute_task_name(&mut self) -> String {
        self.key = Some(match self.root.get_tag() {
            None => {
                let mut relative_path: String = "".to_string();
                for record in self.ancestors.iter().rev() {
                    if let Some(ref t) = record.tag {
                        relative_path = format!("{}__{}", relative_path, t);
                        break;
                    }
                    if record.ix > 0 {
                        relative_path = format!(
                            "{}__{}_{}",
                            relative_path,
                            to_snake_case(&record.object_type),
                            record.ix
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
        let key = self.key.as_ref().unwrap();
        match key.len() {
            0 => to_snake_case(&self.get_name()),
            _ => format!(
                "{}__{}",
                to_snake_case(&self.get_name()),
                self.key.as_ref().unwrap()
            ),
        }
    }
}

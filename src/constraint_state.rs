use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::{AllConstraintsSatisfiability, Constraint};
use crate::dialect::Dialect;
use crate::object::TAoristObject;
use crate::python::{Formatted, ParameterTuple, SimpleIdentifier, StringLiteral, AST};
use anyhow::{bail, Result};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash)]
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
    pub fn compute_relative_path(ancestors: &Vec<AncestorRecord>) -> String {
        let mut relative_path: String = "".to_string();
        for record in ancestors.iter().rev() {
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
}

impl<'a> ConstraintState<'a> {
    pub fn requires_program(&self) -> Result<bool> {
        self.constraint.read().unwrap().requires_program()
    }
    pub fn get_dependencies(&self) -> Result<Vec<Uuid>> {
        let mut dependencies = Vec::new();
        for dep in &self.satisfied_dependencies {
            dependencies.push(dep.read().unwrap().get_constraint_uuid()?);
        }
        Ok(dependencies)
    }
    pub fn get_task_call(&self) -> Result<AST> {
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
            _ => bail!("Dialect not supported for task call: {:?}", self.dialect),
        }
    }
    pub fn get_args_vec(&self) -> Result<Vec<AST>> {
        match (&self.params, &self.dialect) {
            (Some(ref p), Some(Dialect::Python(_))) => Ok(p.get_args()),
            (None, Some(Dialect::Python(_))) => Ok(Vec::new()),
            (_, Some(Dialect::Presto(_))) => Ok(Vec::new()),
            (_, Some(Dialect::Bash(_))) => Ok(Vec::new()),
            (_, None) => Ok(vec![AST::StringLiteral(StringLiteral::new_wrapped(
                self.constraint.read().unwrap().get_name().clone(),
                false,
            ))]),
            _ => bail!("Dialect not supported for args vec: {:?}", self.dialect),
        }
    }
    pub fn get_kwargs_map(&self) -> Result<LinkedHashMap<String, AST>> {
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
            _ => bail!("Dialect not supported for kwargs map: {:?}", self.dialect),
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
    pub fn get_constraint_uuid(&self) -> Result<Uuid> {
        Ok(self.constraint.read().unwrap().get_uuid()?.clone())
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
        assert!(self.ancestors.len() > 0);
        params.set_ancestors(self.ancestors.clone());
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
    ) -> Result<Self> {
        let arc = constraint.clone();
        let x = arc.read().unwrap();
        let root_uuid = x.get_root_uuid()?;
        let guard = concepts.read().unwrap();
        let root = guard
            .get(&(root_uuid.clone(), x.root.clone()))
            .unwrap()
            .clone();
        let mut dependencies: LinkedHashSet<(Uuid, String)> = LinkedHashSet::new();
        for constraint in x.get_downstream_constraints()? {
            let entry = constraint.read().unwrap();
            dependencies.insert((entry.get_uuid()?, entry.root.clone()));
        }

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
        Ok(Self {
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
        })
    }
    fn compute_task_key(&mut self) -> String {
        self.key = Some(match self.root.get_tag() {
            None => AncestorRecord::compute_relative_path(&self.ancestors),
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
    pub fn shorten_task_names(
        constraints: &LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        existing_names: &mut HashSet<String>,
    ) {
        let mut task_names: Vec<(String, Arc<RwLock<ConstraintState<'a>>>)> = Vec::new();
        for constraint in constraints.values() {
            let mut write = constraint.write().unwrap();
            write.compute_task_key();
            let fqn = write.get_fully_qualified_task_name();
            drop(write);
            task_names.push((fqn, constraint.clone()));
        }
        loop {
            let mut should_continue = false;

            let mut proposed_names: Vec<String> = task_names.iter().map(|x| x.0.clone()).collect();
            let mut new_task_names: HashSet<String> = HashSet::new();

            for name in proposed_names.clone().into_iter() {
                if new_task_names.contains(&name) {
                    panic!("Duplicated task name: {}", name);
                }
                new_task_names.insert(name);
            }

            for i in 0..task_names.len() {
                let task_name = proposed_names.get(i).unwrap().clone();
                let new_name = Self::get_shorter_task_name(task_name.clone());

                if new_task_names.contains(&new_name) || existing_names.contains(&new_name) {
                    should_continue = false;
                    break;
                }
                if new_name != task_name {
                    should_continue = true;
                    new_task_names.insert(new_name.clone());
                    proposed_names[i] = new_name;
                }
            }
            if !should_continue {
                break;
            }
            for i in 0..task_names.len() {
                task_names[i].0 = proposed_names[i].clone();
            }
        }
        for (name, rw) in task_names {
            let mut write = rw.write().unwrap();
            existing_names.insert(name.clone());
            write.set_task_name(name.replace("____", "__"));
        }
    }
    fn get_shorter_task_name(task_name: String) -> String {
        let splits = task_name
            .split("__")
            .map(|x| x.to_string())
            .filter(|x| x.len() > 0)
            .collect::<Vec<String>>();
        let mut new_name = task_name.to_string();
        if splits.len() > 2 {
            new_name = format!(
                "{}__{}",
                splits[0].to_string(),
                splits[2..]
                    .iter()
                    .map(|x| x.clone())
                    .collect::<Vec<String>>()
                    .join("__")
            )
            .to_string();
        } else if splits.len() == 2 {
            new_name = splits[0].to_string();
        } else {
            let splits_inner = splits[0]
                .split("_")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if splits_inner.len() > 2 {
                new_name = format!(
                    "{}_{}",
                    splits_inner[0].to_string(),
                    splits_inner[2..]
                        .iter()
                        .map(|x| x.clone())
                        .collect::<Vec<String>>()
                        .join("_")
                )
                .to_string();
            }
        }
        new_name
    }
}

use crate::concept::Ancestry;
use crate::constraint::OuterConstraint;
use crate::context::Context;
use crate::dialect::Dialect;
use crate::parameter_tuple::ParameterTuple;
use crate::program::TOuterProgram;
use crate::task_name_shortener::TaskNameShortener;
use anyhow::{bail, Result};
use aorist_ast::{AncestorRecord, Formatted, SimpleIdentifier, StringLiteral, AST};
use aorist_primitives::TConceptEnum;
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{level_enabled, trace, Level};
use uuid::Uuid;

pub struct ConstraintState<'a, T: OuterConstraint<'a>, P: TOuterProgram<TAncestry = T::TAncestry>> {
    dialect: Option<Dialect>,
    pub key: Option<String>,
    name: String,
    pub satisfied: bool,
    pub satisfied_dependencies: Vec<Arc<RwLock<ConstraintState<'a, T, P>>>>,
    pub unsatisfied_dependencies: LinkedHashSet<(Uuid, String)>,
    constraint: Arc<RwLock<T>>,
    root: <<T as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
    // these are concept ancestors
    // TODO: change this to Vec<Concept<'a>>
    ancestors: Vec<AncestorRecord>,
    preamble: Option<String>,
    call: Option<String>,
    params: Option<ParameterTuple>,
    task_name: Option<String>,
    context: Context,
}
impl<'a, T: OuterConstraint<'a>, P: TOuterProgram<TAncestry = T::TAncestry>>
    ConstraintState<'a, T, P>
{
    pub fn mark_dependency_as_satisfied(
        &mut self,
        dependency: &Arc<RwLock<ConstraintState<'a, T, P>>>,
        uuid: &(Uuid, String),
    ) {
        let dependency_context = &(*dependency.read().unwrap()).context;
        self.satisfied_dependencies.push(dependency.clone());
        self.context.insert(dependency_context);
        assert!(self.unsatisfied_dependencies.remove(uuid));
    }
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
    pub fn get_root(&self) -> <<T as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept {
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
    pub fn find_best_program<'b>(
        preferences: &Vec<Dialect>,
        programs: &'b Vec<P>,
    ) -> Option<&'b P> {
        for dialect in preferences {
            for program in programs {
                if program.get_dialect() == dialect.clone() {
                    return Some(&program);
                }
            }
        }
        None
    }
    pub fn satisfy(
        &mut self,
        preferences: &Vec<Dialect>,
        ancestry: &<T as OuterConstraint<'a>>::TAncestry,
        programs: &Vec<P>,
    ) {
        let best_program = Self::find_best_program(preferences, programs);
        if let Some(program) = best_program {
            let (preamble, call, params, dialect) =
                program.compute_args(self.root.clone(), ancestry, &mut self.context);
            self.preamble = Some(preamble);
            self.call = Some(call);
            self.params = Some(params);
            self.dialect = Some(dialect);
        } else {
            panic!("Could not find any program for constraint {}.", self.constraint.read().unwrap().get_name());
        }
    }
    pub fn new(
        constraint: Arc<RwLock<T>>,
        concepts: Arc<
            RwLock<
                HashMap<
                    (Uuid, String),
                    <<T as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
                >,
            >,
        >,
        concept_ancestors: &HashMap<(Uuid, String), Vec<AncestorRecord>>,
    ) -> Result<Self> {
        let arc = constraint.clone();
        let x = arc.read().unwrap();
        let root_uuid = x.get_root_uuid()?;
        let guard = concepts.read().unwrap();
        let root = guard
            .get(&(root_uuid.clone(), x.get_root()))
            .unwrap()
            .clone();
        let mut dependencies: LinkedHashSet<(Uuid, String)> = LinkedHashSet::new();
        for constraint in x.get_downstream_constraints()? {
            let entry = constraint.read().unwrap();
            dependencies.insert((entry.get_uuid()?, entry.get_root()));
        }

        if level_enabled!(Level::TRACE) {
            trace!(
                "Constraint {} on {:?} {} has the following dependencies",
                x.get_name(),
                x.get_root_type_name(),
                &root_uuid
            );
            for dependency in dependencies.iter() {
                trace!("  {:?}", dependency);
            }
        }
        let ancestors = concept_ancestors
            .get(&(root_uuid, x.get_root()))
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
            // will accumulate dependencies' contexts as they are satisfied
            context: Context::new(),
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
        constraints: &LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, T, P>>>>,
        existing_names: &mut HashSet<String>,
    ) {
        let mut task_names: Vec<(String, Arc<RwLock<ConstraintState<'a, T, P>>>)> = Vec::new();
        for constraint in constraints.values() {
            let mut write = constraint.write().unwrap();
            write.compute_task_key();
            let fqn = write.get_fully_qualified_task_name();
            drop(write);
            task_names.push((fqn, constraint.clone()));
        }
        let to_shorten_task_names = task_names.iter().map(|(x, _)| x.clone()).collect();
        let shortened_task_names_1 =
            TaskNameShortener::new(to_shorten_task_names, "____".to_string()).run();
        let shortened_task_names_2 =
            TaskNameShortener::new(shortened_task_names_1, "_".to_string()).run();
        for (i, (_, rw)) in task_names.iter().enumerate() {
            let name = shortened_task_names_2.get(i).unwrap().clone();
            let mut write = rw.write().unwrap();
            existing_names.insert(name.clone());
            write.set_task_name(name.replace("____", "__"));
        }
    }
}

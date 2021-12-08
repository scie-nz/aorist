
use crate::concept::Ancestry;
use crate::constraint::OuterConstraint;
use crate::dialect::Dialect;
use crate::parameter_tuple::ParameterTuple;
use crate::program::TOuterProgram;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use anyhow::{bail, Result};
use aorist_ast::{AncestorRecord, Formatted, SimpleIdentifier, StringLiteral, AST};
use aorist_primitives::{AString, AVec, Context, TConceptEnum};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet};
use tracing::{debug, level_enabled, trace, Level};
use uuid::Uuid;

pub struct ConstraintState<'a, T: OuterConstraint<'a>, P: TOuterProgram<TAncestry = T::TAncestry>> {
    dialect: Option<Dialect>,
    pub key: Option<AString>,
    name: AString,
    pub satisfied: bool,
    pub satisfied_dependencies: AVec<RArc<RRwLock<ConstraintState<'a, T, P>>>>,
    pub unsatisfied_dependencies: LinkedHashSet<(Uuid, AString)>,
    constraint: RArc<RRwLock<T>>,
    root: <<T as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
    // these are concept ancestors
    // TODO: change this to AVec<Concept<'a>>
    ancestors: AVec<AncestorRecord>,
    preamble: Option<AString>,
    call: Option<AString>,
    params: Option<ParameterTuple>,
    task_name: Option<AString>,
    context: Context,
}
impl<'a, T: OuterConstraint<'a>, P: TOuterProgram<TAncestry = T::TAncestry>>
    ConstraintState<'a, T, P>
{
    pub fn mark_dependency_as_satisfied(
        &mut self,
        dependency: &RArc<RRwLock<ConstraintState<'a, T, P>>>,
        uuid: &(Uuid, AString),
    ) {
        let dependency_name = dependency.read().get_name();
        let dependency_context = &(*dependency.read()).context;
        self.satisfied_dependencies.push(dependency.clone());
        self.context
            .insert(dependency_context, dependency_name.as_str().into());
        assert!(self.unsatisfied_dependencies.remove(uuid));
        debug!("Marked dependency {} as satisfied.", dependency_name);
    }
    pub fn requires_program(&self) -> Result<bool> {
        self.constraint.read().requires_program()
    }
    pub fn get_dependencies(&self) -> Result<AVec<Uuid>> {
        let mut dependencies = AVec::new();
        for dep in self.satisfied_dependencies.iter() {
            dependencies.push(dep.read().get_constraint_uuid()?);
        }
        Ok(dependencies)
    }
    pub fn get_task_call(&self) -> Result<AST> {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                self.get_call().unwrap(),
            ))),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => Ok(AST::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("ShellTask".into()),
            )),
            None => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "ConstantTask".into(),
            ))),
            _ => bail!("Dialect not supported for task call: {:?}", self.dialect),
        }
    }
    pub fn get_args_vec(&self) -> Result<AVec<AST>> {
        match (&self.params, &self.dialect) {
            (Some(ref p), Some(Dialect::Python(_))) => Ok(p.get_args()),
            (None, Some(Dialect::Python(_))) => Ok(AVec::new()),
            (_, Some(Dialect::Presto(_))) => Ok(AVec::new()),
            (_, Some(Dialect::Bash(_))) => Ok(AVec::new()),
            (_, None) => Ok(vec![AST::StringLiteral(StringLiteral::new_wrapped(
                self.constraint.read().get_name().clone(),
                false,
            ))].into_iter().collect()),
            _ => bail!("Dialect not supported for args vec: {:?}", self.dialect),
        }
    }
    pub fn get_kwargs_map(&self) -> Result<LinkedHashMap<AString, AST>> {
        match &self.dialect {
            Some(Dialect::Python(_)) => match self.params {
                Some(ref p) => Ok(p.get_kwargs()),
                None => Ok(LinkedHashMap::new()),
            },
            Some(Dialect::Presto(_)) => {
                let raw_command = format!("presto -e '{}'", self.get_call().unwrap().clone());
                let format_string = StringLiteral::new_wrapped(raw_command.as_str().into(), true);
                let command = match self.params {
                    Some(ref p) => AST::Formatted(Formatted::new_wrapped(
                        AST::StringLiteral(format_string),
                        p.get_kwargs(),
                    )),
                    None => AST::StringLiteral(format_string),
                };
                let mut keywords: LinkedHashMap<AString, AST> = LinkedHashMap::new();
                keywords.insert("command".into(), command);
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
                let mut keywords: LinkedHashMap<AString, AST> = LinkedHashMap::new();
                keywords.insert("command".into(), command);
                Ok(keywords)
            }
            None => Ok(LinkedHashMap::new()),
            _ => bail!("Dialect not supported for kwargs map: {:?}", self.dialect),
        }
    }
    pub fn set_task_name(&mut self, name: AString) {
        self.task_name = Some(name)
    }
    pub fn get_task_name(&self) -> AString {
        self.task_name.as_ref().unwrap().clone()
    }
    pub fn get_satisfied_dependency_keys(&self) -> AVec<AString> {
        self.satisfied_dependencies
            .iter()
            .map(|x| x.read().get_task_name())
            .collect()
    }
    pub fn get_name(&self) -> AString {
        self.name.clone()
    }
    #[allow(dead_code)]
    pub fn get_root(&self) -> <<T as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept {
        self.root.clone()
    }
    pub fn get_constraint_uuid(&self) -> Result<Uuid> {
        Ok(self.constraint.read().get_uuid()?.clone())
    }
    #[allow(dead_code)]
    pub fn get_root_uuid(&self) -> Uuid {
        self.root.get_uuid().clone()
    }
    pub fn get_root_type(&self) -> AString {
        self.root.get_type()
    }
    pub fn get_ancestors(&self) -> AVec<AncestorRecord> {
        self.ancestors.clone()
    }
    pub fn get_preamble(&self) -> Option<AString> {
        self.preamble.clone()
    }
    pub fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    pub fn get_params(&self) -> Option<ParameterTuple> {
        self.params.clone()
    }
    pub fn get_call(&self) -> Option<AString> {
        self.call.clone()
    }
    pub fn get_key(&self) -> Option<AString> {
        self.key.clone()
    }
    pub fn find_best_program<'b>(
        preferences: &AVec<Dialect>,
        programs: &'b AVec<P>,
    ) -> Option<&'b P> {
        for dialect in preferences.iter() {
            for program in programs.iter() {
                if program.get_dialect() == dialect.clone() {
                    return Some(&program);
                }
            }
        }
        None
    }
    pub fn satisfy(
        &mut self,
        preferences: &AVec<Dialect>,
        ancestry: &<T as OuterConstraint<'a>>::TAncestry,
        programs: &AVec<P>,
    ) {
        let best_program = Self::find_best_program(preferences, programs);
        if let Some(program) = best_program {
            let (preamble, call, params, dialect) = program.compute_args(
                self.root.clone(),
                ancestry,
                &mut self.context,
                self.constraint.clone(),
            );
            self.preamble = Some(preamble);
            self.call = Some(call);
            self.params = Some(params);
            self.dialect = Some(dialect);
        } else {
            panic!(
                "Could not find any program for constraint {}.",
                self.constraint.read().get_name()
            );
        }
    }
    pub fn get_dedup_key(&self) -> (AString, AString, ParameterTuple, Option<Dialect>) {
        (
            self.preamble.as_ref().unwrap().clone(),
            self.call.as_ref().unwrap().clone(),
            self.params.as_ref().unwrap().clone(),
            self.dialect.clone(),
        )
    }
    pub fn new(
        constraint: RArc<RRwLock<T>>,
        concepts: RArc<
            RRwLock<
                HashMap<
                    (Uuid, AString),
                    <<T as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
                >,
            >,
        >,
        concept_ancestors: &HashMap<(Uuid, AString), AVec<AncestorRecord>>,
    ) -> Result<Self> {
        let arc = constraint.clone();
        let x = arc.read();
        let root_uuid = x.get_root_uuid()?;
        let guard = concepts.read();
        let root = guard
            .get(&(root_uuid.clone(), x.get_root()))
            .unwrap()
            .clone();
        let mut dependencies: LinkedHashSet<(Uuid, AString)> = LinkedHashSet::new();
        for constraint in x.get_downstream_constraints()? {
            let entry = constraint.read();
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
            satisfied_dependencies: AVec::new(),
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
    pub fn compute_task_key(&mut self) -> AString {
        self.key = Some(match self.root.get_tag() {
            None => AncestorRecord::compute_relative_path(&self.ancestors),
            Some(t) => t,
        });
        self.key.as_ref().unwrap().clone()
    }
    pub fn get_fully_qualified_task_name(&self) -> AString {
        let key = self.key.as_ref().unwrap();
        let name = match key.len() {
            0 => to_snake_case(self.get_name().as_str()),
            _ => format!(
                "{}__{}",
                to_snake_case(self.get_name().as_str()),
                self.key.as_ref().unwrap()
            ),
        };
        format!(
            "{}__{}",
            name,
            self.constraint
                .read()
                .get_uuid()
                .unwrap()
                .to_string()
                .split("-")
                .take(1)
                .next()
                .unwrap()
        )
        .as_str()
        .into()
    }
    pub fn shorten_task_names(
        constraints: &LinkedHashMap<(Uuid, AString), RArc<RRwLock<ConstraintState<'a, T, P>>>>,
        _existing_names: &mut HashSet<AString>,
    ) {
        let mut task_names: AVec<(AString, RArc<RRwLock<ConstraintState<'a, T, P>>>)> = AVec::new();
        for constraint in constraints.values() {
            let mut write = constraint.write();
            write.compute_task_key();
            let fqn = write.get_fully_qualified_task_name();
            write.set_task_name(fqn.clone());
            drop(write);
            task_names.push((fqn, constraint.clone()));
        }
        /*let mut to_shorten_task_names = task_names.iter().map(|(x, _)| x.clone()).collect();
        let shortened_task_names_1 =
            TaskNameShortener::new(to_shorten_task_names, "____".into(),
                                   existing_names.clone()).run();
        let shortened_task_names_2 =
            TaskNameShortener::new(shortened_task_names_1, "_".into(),
                                   existing_names.clone()).run();
        for (i, (_, rw)) in task_names.iter().enumerate() {
            let name = shortened_task_names_1.get(i).unwrap().clone();
            let mut write = rw.write();
            existing_names.insert(name.clone());
            write.set_task_name(name.replace("____", "__"));
        }*/
    }
}

use crate::constraint::OuterConstraint;
use crate::parameter_tuple::ParameterTuple;
use crate::program::TOuterProgram;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use abi_stable::std_types::ROption;
use anyhow::{bail, Result};
use aorist_ast::{AncestorRecord, Formatted, SimpleIdentifier, StringLiteral, AST};
use aorist_primitives::Dialect;
use aorist_primitives::{Context, ToplineConcept, Ancestry};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::{AString, AVec, ATaskId};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet};
use tracing::{debug, level_enabled, trace, Level};

pub struct ConstraintState<'a, T: OuterConstraint<'a>, P: TOuterProgram<TAncestry = T::TAncestry>> {
    dialect: AOption<Dialect>,
    pub key: AOption<AString>,
    name: AString,
    pub satisfied: bool,
    pub satisfied_dependencies: AVec<RArc<RRwLock<ConstraintState<'a, T, P>>>>,
    pub unsatisfied_dependencies: LinkedHashSet<ATaskId>,
    constraint: RArc<RRwLock<T>>,
    root: <<T as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
    // these are concept ancestors
    // TODO: change this to AVec<Concept<'a>>
    ancestors: AVec<AncestorRecord>,
    preamble: AOption<AString>,
    call: AOption<AString>,
    params: AOption<ParameterTuple>,
    task_name: AOption<AString>,
    context: Context,
}
impl<'a, T: OuterConstraint<'a>, P: TOuterProgram<TAncestry = T::TAncestry>>
    ConstraintState<'a, T, P>
{
    pub fn mark_dependency_as_satisfied(
        &mut self,
        dependency: &RArc<RRwLock<ConstraintState<'a, T, P>>>,
        uuid: &ATaskId,
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
        Ok(self.constraint.read().requires_program())
    }
    pub fn get_dependencies(&self) -> Result<AVec<AUuid>> {
        let mut dependencies = AVec::new();
        for dep in self.satisfied_dependencies.iter() {
            dependencies.push(dep.read().get_constraint_uuid()?);
        }
        Ok(dependencies)
    }
    pub fn get_task_call(&self) -> Result<AST> {
        match self.dialect {
            AOption(ROption::RSome(Dialect::Python(_))) => Ok(AST::SimpleIdentifier(
                SimpleIdentifier::new_wrapped(self.get_call().unwrap()),
            )),
            AOption(ROption::RSome(Dialect::Bash(_)))
            | AOption(ROption::RSome(Dialect::Presto(_))) => Ok(AST::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("ShellTask".into()),
            )),
            AOption(ROption::RNone) => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "ConstantTask".into(),
            ))),
            _ => bail!("Dialect not supported for task call: {:?}", self.dialect),
        }
    }
    pub fn get_args_vec(&self) -> Result<AVec<AST>> {
        match (&self.params, &self.dialect) {
            (AOption(ROption::RSome(ref p)), AOption(ROption::RSome(Dialect::Python(_)))) => {
                Ok(p.get_args())
            }
            (AOption(ROption::RNone), AOption(ROption::RSome(Dialect::Python(_)))) => {
                Ok(AVec::new())
            }
            (_, AOption(ROption::RSome(Dialect::Presto(_)))) => Ok(AVec::new()),
            (_, AOption(ROption::RSome(Dialect::Bash(_)))) => Ok(AVec::new()),
            (_, AOption(ROption::RNone)) => Ok(vec![AST::StringLiteral(
                StringLiteral::new_wrapped(self.constraint.read().get_name().clone(), false),
            )]
            .into_iter()
            .collect()),
            _ => bail!("Dialect not supported for args vec: {:?}", self.dialect),
        }
    }
    pub fn get_kwargs_map(&self) -> Result<LinkedHashMap<AString, AST>> {
        match &self.dialect {
            AOption(ROption::RSome(Dialect::Python(_))) => match self.params {
                AOption(ROption::RSome(ref p)) => Ok(p.get_kwargs()),
                AOption(ROption::RNone) => Ok(LinkedHashMap::new()),
            },
            AOption(ROption::RSome(Dialect::Presto(_))) => {
                let raw_command = format!("presto -e '{}'", self.get_call().unwrap().clone());
                let format_string = StringLiteral::new_wrapped(raw_command.as_str().into(), true);
                let command = match self.params {
                    AOption(ROption::RSome(ref p)) => AST::Formatted(Formatted::new_wrapped(
                        AST::StringLiteral(format_string),
                        p.get_kwargs(),
                    )),
                    AOption(ROption::RNone) => AST::StringLiteral(format_string),
                };
                let mut keywords: LinkedHashMap<AString, AST> = LinkedHashMap::new();
                keywords.insert("command".into(), command);
                Ok(keywords)
            }
            AOption(ROption::RSome(Dialect::Bash(_))) => {
                let format_string =
                    AST::StringLiteral(StringLiteral::new_wrapped(self.get_call().unwrap(), false));
                let command = match self.params {
                    AOption(ROption::RSome(ref p)) => {
                        AST::Formatted(Formatted::new_wrapped(format_string, p.get_kwargs()))
                    }
                    AOption(ROption::RNone) => format_string,
                };
                let mut keywords: LinkedHashMap<AString, AST> = LinkedHashMap::new();
                keywords.insert("command".into(), command);
                Ok(keywords)
            }
            AOption(ROption::RNone) => Ok(LinkedHashMap::new()),
            _ => bail!("Dialect not supported for kwargs map: {:?}", self.dialect),
        }
    }
    pub fn set_task_name(&mut self, name: AString) {
        self.task_name = AOption(ROption::RSome(name))
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
    pub fn get_constraint_uuid(&self) -> Result<AUuid> {
        Ok(self.constraint.read().get_uuid().clone())
    }
    #[allow(dead_code)]
    pub fn get_root_uuid(&self) -> AUuid {
        self.root.get_uuid().clone()
    }
    pub fn get_root_type(&self) -> AString {
        self.root.get_type()
    }
    pub fn get_ancestors(&self) -> AVec<AncestorRecord> {
        self.ancestors.clone()
    }
    pub fn get_preamble(&self) -> AOption<AString> {
        self.preamble.clone()
    }
    pub fn get_dialect(&self) -> AOption<Dialect> {
        self.dialect.clone()
    }
    pub fn get_params(&self) -> AOption<ParameterTuple> {
        self.params.clone()
    }
    pub fn get_call(&self) -> AOption<AString> {
        self.call.clone()
    }
    pub fn get_key(&self) -> AOption<AString> {
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
            self.preamble = AOption(ROption::RSome(preamble));
            self.call = AOption(ROption::RSome(call));
            self.params = AOption(ROption::RSome(params));
            self.dialect = AOption(ROption::RSome(dialect));
        } else {
            panic!(
                "Could not find any program for constraint {}.",
                self.constraint.read().get_name()
            );
        }
    }
    pub fn get_dedup_key(&self) -> (AString, AString, ParameterTuple, AOption<Dialect>) {
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
                    ATaskId,
                    <<T as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
                >,
            >,
        >,
        concept_ancestors: &HashMap<ATaskId, AVec<AncestorRecord>>,
    ) -> Result<Self> {
        let arc = constraint.clone();
        let x = arc.read();
        let root_uuid = x.get_root_uuid();
        let guard = concepts.read();
        let task_id = ATaskId::new(root_uuid.clone(), x.get_root());
        let root = guard
            .get(&task_id)
            .unwrap()
            .clone();
        let dependencies = constraint.read().get_dependencies();

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
            .get(&task_id)
            .unwrap()
            .clone();
        Ok(Self {
            dialect: AOption(ROption::RNone),
            key: AOption(ROption::RNone),
            name: x.get_name().clone(),
            satisfied: false,
            unsatisfied_dependencies: dependencies.into_iter().collect(),
            satisfied_dependencies: AVec::new(),
            constraint,
            root,
            ancestors: ancestors.clone(),
            preamble: AOption(ROption::RNone),
            call: AOption(ROption::RNone),
            params: AOption(ROption::RNone),
            task_name: AOption(ROption::RNone),
            // will accumulate dependencies' contexts as they are satisfied
            context: Context::new(),
        })
    }
    pub fn compute_task_key(&mut self) -> AString {
        self.key = AOption(ROption::RSome(match self.root.get_tag() {
            AOption(ROption::RNone) => AncestorRecord::compute_relative_path(&self.ancestors),
            AOption(ROption::RSome(t)) => t,
        }));
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
        constraints: &LinkedHashMap<ATaskId, RArc<RRwLock<ConstraintState<'a, T, P>>>>,
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

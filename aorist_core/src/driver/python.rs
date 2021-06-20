#![allow(dead_code)]
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::SatisfiableOuterConstraint;
use crate::constraint_state::ConstraintState;
use crate::dialect::{Bash, Dialect, Presto, Python, R};
use crate::driver::{ConstraintsBlockMap, Driver};
use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, FlowBuilderBase, FlowBuilderMaterialize};
use crate::parameter_tuple::ParameterTuple;
#[cfg(feature = "python")]
use crate::python::{
    PythonBasedConstraintBlock, PythonFlowBuilderInput, PythonImport, PythonPreamble,
};
#[cfg(feature = "r")]
use crate::r::{RBasedConstraintBlock, RFlowBuilderInput, RImport, RPreamble};
use crate::universe::Universe;
use anyhow::Result;
use aorist_ast::{AncestorRecord, SimpleIdentifier, AST};
use aorist_primitives::{OuterConstraint, TBuilder, Ancestry};
use aorist_primitives::{TAoristObject, TConceptEnum, TConstraintEnum};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use tracing::{debug, level_enabled, trace, Level};
use uuid::Uuid;

pub struct PythonBasedDriver<'a, 'b, D, C>
where
    D: FlowBuilderBase,
    <D as FlowBuilderBase>::T:
        ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble> + 'a,
    C: OuterConstraint<'a, 'b, TAncestry=ConceptAncestry<'a>> + SatisfiableOuterConstraint<'a, 'b> + 'b,
    'a : 'b,
{
    pub concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>, 
    constraints: LinkedHashMap<(Uuid, String), Arc<RwLock<C>>>,
    satisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, 'b, C>>>>,
    blocks: Vec<PythonBasedConstraintBlock<'a, 'b, D::T, C>>,
    ancestry: Arc<ConceptAncestry<'a>>,
    dag_type: PhantomData<D>,
    endpoints: EndpointConfig,
    constraint_explanations: HashMap<String, (Option<String>, Option<String>)>,
    ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>>,
    topline_constraint_names: LinkedHashSet<String>,
    _lt_phantom: PhantomData<&'b ()>,
}

impl<'a, 'b, D, C> Driver<'a, 'b, D, C> for PythonBasedDriver<'a, 'b, D, C>
where
    'a : 'b,
    D: FlowBuilderBase,
    D: FlowBuilderMaterialize<BuilderInputType = PythonFlowBuilderInput>,
    <D as FlowBuilderBase>::T:
        ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble> + 'a,
    C: OuterConstraint<'a, 'b, TAncestry=ConceptAncestry<'a>> + SatisfiableOuterConstraint<'a, 'b> + 'b,
    //Self::CB: 'b,
{
    type CB = PythonBasedConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>;

    fn get_preferences(&self) -> Vec<Dialect> {
        vec![
            Dialect::Python(Python::new(vec![])),
            Dialect::Presto(Presto {}),
            Dialect::Bash(Bash {}),
        ]
    }
    fn get_constraint_rwlock(&self, uuid: &(Uuid, String)) -> Arc<RwLock<C>> {
        self.constraints.get(uuid).unwrap().clone()
    }

    fn get_endpoints(&'b self) -> &'b EndpointConfig {
        &self.endpoints
    }

    fn get_ancestry(&self) -> Arc<ConceptAncestry<'a>> {
        self.ancestry.clone()
    }
    fn mark_constraint_state_as_satisfied(
        &mut self,
        id: (Uuid, String),
        state: Arc<RwLock<ConstraintState<'a, 'b, C>>>,
    ) {
        self.satisfied_constraints.insert(id, state.clone());
    }
    fn init_unsatisfied_constraints(&self) -> Result<ConstraintsBlockMap<'a, 'b, C>> {
        Self::get_unsatisfied_constraints(
            &self.constraints,
            self.concepts.clone(),
            &self.ancestors,
            self.topline_constraint_names.clone(),
        )
    }
    fn add_block(
        &mut self,
        constraint_block: PythonBasedConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>,
    ) {
        self.blocks.push(constraint_block);
    }
    fn get_constraint_explanation(
        &self,
        constraint_name: &String,
    ) -> (Option<String>, Option<String>) {
        self.constraint_explanations
            .get(constraint_name)
            .unwrap()
            .clone()
    }
    fn get_blocks(&'b self) -> &'b Vec<Self::CB> {
        &self.blocks
    }
    fn get_dependencies(&self) -> Vec<String> {
        self.satisfied_constraints
            .values()
            .map(|x| match x.read().unwrap().get_dialect() {
                Some(Dialect::Python(x)) => Some(x.get_pip_requirements()),
                _ => None,
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().into_iter())
            .flatten()
            .collect::<BTreeSet<String>>()
            .into_iter()
            .collect()
    }
    fn _new(
        concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
        constraints: LinkedHashMap<(Uuid, String), Arc<RwLock<C>>>,
        ancestry: Arc<ConceptAncestry<'a>>,
        endpoints: EndpointConfig,
        ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>>,
        topline_constraint_names: LinkedHashSet<String>,
    ) -> Self {
        Self {
            concepts,
            constraints,
            satisfied_constraints: HashMap::new(),
            blocks: Vec::new(),
            ancestry,
            dag_type: PhantomData,
            endpoints,
            constraint_explanations: <<C as OuterConstraint<'a, 'b>>::TEnum as TConstraintEnum<'a, 'b>>::get_explanations(),
            ancestors,
            topline_constraint_names,
            _lt_phantom: PhantomData,
        }
    }
}

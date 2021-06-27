#![allow(dead_code)]
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint_block::ConstraintBlock;
use crate::constraint_state::ConstraintState;
use crate::dialect::{Bash, Dialect, Presto, Python};
use crate::driver::{ConstraintsBlockMap, Driver};
use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, FlowBuilderBase, FlowBuilderMaterialize, PythonBasedFlowBuilder};
use crate::python::{
    PythonBasedConstraintBlock, PythonFlowBuilderInput, PythonImport, PythonPreamble,
};
use anyhow::Result;
use aorist_ast::AncestorRecord;
use crate::constraint::TConstraintEnum;
use crate::constraint::{OuterConstraint, TBuilder};
use aorist_primitives::{Ancestry, TConceptEnum, AoristConcept, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use crate::program::{Program, TOuterProgram};

pub struct PythonBasedDriver<'a, B, D, U, C, A, P>
where
    U: AoristConcept + AoristUniverse,
    B: TBuilder<'a, TEnum = C, TAncestry = A>,
    D: FlowBuilderBase<U> + PythonBasedFlowBuilder<U>,
    <D as FlowBuilderBase<U>>::T: 'a,
    <D as FlowBuilderBase<U>>::T:
        ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble> + 'a,
    A: Ancestry,
    C: TConceptEnum<TUniverse = U>,
    <B as TBuilder<'a>>::OuterType: OuterConstraint<'a, TAncestry = A>,
    <<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry: Ancestry<TConcept = C>,
    <<<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept:
        TConceptEnum<TUniverse = U>,
    P: TOuterProgram<TAncestry = A>,
{
    pub concepts: Arc<RwLock<HashMap<(Uuid, String), C>>>,
    constraints: LinkedHashMap<(Uuid, String), Arc<RwLock<B::OuterType>>>,
    satisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, B::OuterType, P>>>>,
    blocks: Vec<PythonBasedConstraintBlock<'a, D::T, B::OuterType, U, P>>,
    ancestry: A,
    dag_type: PhantomData<D>,
    endpoints: <U as AoristUniverse>::TEndpoints,
    constraint_explanations: HashMap<String, (Option<String>, Option<String>)>,
    ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>>,
    topline_constraint_names: LinkedHashSet<String>,
    programs: LinkedHashMap<String, Vec<P>>,
}
impl<'a, B, D, U, C, A, P> Driver<'a, B, D, U, C, A, P> for PythonBasedDriver<'a, B, D, U, C, A, P>
where
    U: AoristConcept + AoristUniverse,
    B: TBuilder<'a, TEnum = C, TAncestry = A>,
    D: FlowBuilderBase<U> + PythonBasedFlowBuilder<U>,
    <D as FlowBuilderBase<U>>::T: 'a,
    <D as FlowBuilderBase<U>>::T:
        ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble> + 'a,
    A: Ancestry,
    C: TConceptEnum<TUniverse = U>,
    <B as TBuilder<'a>>::OuterType: OuterConstraint<'a, TAncestry = A>,
    <<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry: Ancestry<TConcept = C>,
    <<<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept:
        TConceptEnum<TUniverse = U>,
    P: TOuterProgram<TAncestry = A>,
{
    type CB = PythonBasedConstraintBlock<'a, <D as FlowBuilderBase<U>>::T, B::OuterType, U, P>;

    fn get_programs_for(&self, constraint_name: &String) -> Vec<P> {
        self.programs.get(constraint_name).unwrap().iter().map(|x| (*x).clone()).collect()
    }
    fn get_preferences(&self) -> Vec<Dialect> {
        vec![
            Dialect::Python(Python::new(vec![])),
            Dialect::Presto(Presto {}),
            Dialect::Bash(Bash {}),
        ]
    }
    fn get_constraint_rwlock(&self, uuid: &(Uuid, String)) -> Arc<RwLock<B::OuterType>> {
        self.constraints.get(uuid).unwrap().clone()
    }

    fn get_endpoints(&self) -> <U as AoristUniverse>::TEndpoints {
        self.endpoints.clone()
    }

    fn get_ancestry(&self) -> &A {
        &self.ancestry
    }
    fn mark_constraint_state_as_satisfied(
        &mut self,
        id: (Uuid, String),
        state: Arc<RwLock<ConstraintState<'a, B::OuterType, P>>>,
    ) {
        self.satisfied_constraints.insert(id, state.clone());
    }
    fn init_unsatisfied_constraints(&self) -> Result<ConstraintsBlockMap<'a, B::OuterType, P>> {
        Self::get_unsatisfied_constraints(
            &self.constraints,
            self.concepts.clone(),
            &self.ancestors,
            self.topline_constraint_names.clone(),
        )
    }
    fn add_block(
        &mut self,
        constraint_block: PythonBasedConstraintBlock<'a, <D as FlowBuilderBase<U>>::T, B::OuterType, U, P>,
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
    fn get_blocks(&self) -> &Vec<Self::CB> {
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
        concepts: Arc<RwLock<HashMap<(Uuid, String), C>>>,
        constraints: LinkedHashMap<(Uuid, String), Arc<RwLock<B::OuterType>>>,
        ancestry: A,
        endpoints: U::TEndpoints,
        ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>>,
        topline_constraint_names: LinkedHashSet<String>,
        programs: LinkedHashMap<String, Vec<P>>,
    ) -> Self {
        Self {
            concepts,
            constraints,
            satisfied_constraints: HashMap::new(),
            blocks: Vec::new(),
            ancestry,
            dag_type: PhantomData,
            endpoints,
            constraint_explanations: <<B::OuterType as OuterConstraint<'a>>::TEnum as TConstraintEnum<
                'a,
            >>::get_explanations(),
            ancestors,
            topline_constraint_names,
            programs,
        }
    }
}

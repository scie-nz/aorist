use abi_stable::std_types::ROption;
use aorist_util::AOption;

use crate::constraint::TConstraintEnum;
use crate::constraint::{OuterConstraint, TBuilder};
use crate::constraint_state::ConstraintState;
use aorist_primitives::Dialect;
use crate::driver::{ConstraintsBlockMap, Driver};
use crate::flow::{ETLFlow, FlowBuilderBase, PythonBasedFlowBuilder};
use crate::program::TOuterProgram;
use crate::python::{PythonBasedConstraintBlock, PythonImport, PythonPreamble};
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use anyhow::Result;
use aorist_ast::AncestorRecord;
use aorist_util::AUuid;
use aorist_util::{AString, AVec};
use aorist_primitives::{Ancestry, AoristConcept, AoristUniverse, ToplineConcept};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;

pub struct PythonBasedDriver<'a, B, D, U, C, A, P>
where
    U: AoristConcept + AoristUniverse,
    B: TBuilder<'a, TEnum = C, TAncestry = A>,
    D: FlowBuilderBase<U> + PythonBasedFlowBuilder<U>,
    <D as FlowBuilderBase<U>>::T: 'a,
    <D as FlowBuilderBase<U>>::T:
        ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble> + 'a,
    A: Ancestry,
    C: ToplineConcept<TUniverse = U>,
    <B as TBuilder<'a>>::OuterType: OuterConstraint<'a, TAncestry = A>,
    <<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry: Ancestry<TConcept = C>,
    <<<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept:
        ToplineConcept<TUniverse = U>,
    P: TOuterProgram<TAncestry = A>,
{
    pub concepts: RArc<RRwLock<HashMap<(AUuid, AString), C>>>,
    constraints: LinkedHashMap<(AUuid, AString), RArc<RRwLock<B::OuterType>>>,
    satisfied_constraints:
        HashMap<(AUuid, AString), RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>>,
    blocks: AVec<PythonBasedConstraintBlock<'a, D::T, B::OuterType, U, P>>,
    ancestry: A,
    dag_type: PhantomData<D>,
    endpoints: <U as AoristUniverse>::TEndpoints,
    constraint_explanations: HashMap<AString, (AOption<AString>, AOption<AString>)>,
    ancestors: HashMap<(AUuid, AString), AVec<AncestorRecord>>,
    topline_constraint_names: LinkedHashSet<AString>,
    programs: LinkedHashMap<AString, AVec<P>>,
    preferences: AVec<Dialect>,
    render_dependencies: bool,
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
    C: ToplineConcept<TUniverse = U>,
    <B as TBuilder<'a>>::OuterType: OuterConstraint<'a, TAncestry = A>,
    <<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry: Ancestry<TConcept = C>,
    <<<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept:
        ToplineConcept<TUniverse = U>,
    P: TOuterProgram<TAncestry = A>,
{
    type CB = PythonBasedConstraintBlock<'a, <D as FlowBuilderBase<U>>::T, B::OuterType, U, P>;

    fn get_programs_for(&self, constraint_name: &AString) -> AVec<P> {
        match self.programs.get(constraint_name) {
            Some(ref programs) => programs.iter().map(|x| (*x).clone()).collect(),
            None => AVec::new(), //panic!("Cannot find program for {}", constraint_name),
        }
    }
    fn get_preferences(&self) -> AVec<Dialect> {
        self.preferences.clone()
    }
    fn get_constraint_rwlock(&self, uuid: &(AUuid, AString)) -> RArc<RRwLock<B::OuterType>> {
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
        id: (AUuid, AString),
        state: RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>,
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
        constraint_block: PythonBasedConstraintBlock<
            'a,
            <D as FlowBuilderBase<U>>::T,
            B::OuterType,
            U,
            P,
        >,
    ) {
        self.blocks.push(constraint_block);
    }
    fn get_constraint_explanation(
        &self,
        constraint_name: &AString,
    ) -> (AOption<AString>, AOption<AString>) {
        self.constraint_explanations
            .get(constraint_name)
            .unwrap()
            .clone()
    }
    fn get_blocks(&self) -> &AVec<Self::CB> {
        &self.blocks
    }
    fn get_dependencies(&self) -> AVec<AString> {
        self.satisfied_constraints
            .values()
            .map(|x| match x.read().get_dialect() {
                AOption(ROption::RSome(Dialect::Python(x))) => {
                    AOption(ROption::RSome(x.get_pip_requirements()))
                }
                _ => AOption(ROption::RNone),
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().into_iter())
            .flatten()
            .collect::<BTreeSet<AString>>()
            .into_iter()
            .collect()
    }
    fn _new(
        concepts: RArc<RRwLock<HashMap<(AUuid, AString), C>>>,
        constraints: LinkedHashMap<(AUuid, AString), RArc<RRwLock<B::OuterType>>>,
        ancestry: A,
        endpoints: U::TEndpoints,
        ancestors: HashMap<(AUuid, AString), AVec<AncestorRecord>>,
        topline_constraint_names: LinkedHashSet<AString>,
        programs: LinkedHashMap<AString, AVec<P>>,
        preferences: AVec<Dialect>,
        render_dependencies: bool,
    ) -> Self {
        Self {
            concepts,
            constraints,
            satisfied_constraints: HashMap::new(),
            blocks: AVec::new(),
            ancestry,
            dag_type: PhantomData,
            endpoints,
            constraint_explanations: <<B::OuterType as OuterConstraint<'a>>::TEnum as TConstraintEnum<
                'a,
            >>::get_explanations(),
            ancestors,
            topline_constraint_names,
            programs,
            preferences,
            render_dependencies
        }
    }
    fn get_render_dependencies(&self) -> bool {
        self.render_dependencies
    }
}

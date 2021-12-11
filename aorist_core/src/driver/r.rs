#![allow(dead_code)]
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::SatisfiableOuterConstraint;
use crate::constraint_state::ConstraintState;
use crate::dialect::{Bash, Dialect, Python, R};
use crate::driver::{ConstraintsBlockMap, Driver};
use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, FlowBuilderBase, FlowBuilderMaterialize};
use crate::r::{RBasedConstraintBlock, RFlowBuilderInput, RImport, RPreamble};
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use anyhow::Result;
use aorist_ast::AncestorRecord;
use aorist_primitives::OuterConstraint;
use aorist_primitives::TConstraintEnum;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

pub struct RBasedDriver<'a, 'b, D, C>
where
    D: FlowBuilderBase,
    <D as FlowBuilderBase>::T: ETLFlow<ImportType = RImport, PreambleType = RPreamble> + 'a,
    C: OuterConstraint<'a, 'b, TAncestry = ConceptAncestry<'a>>
        + SatisfiableOuterConstraint<'a, 'b>
        + 'b,
    'a: 'b,
{
    pub concepts: RArc<RRwLock<HashMap<(Uuid, AString), Concept<'a>>>>,
    constraints: LinkedHashMap<(Uuid, AString), RArc<RRwLock<C>>>,
    satisfied_constraints: HashMap<(Uuid, AString), RArc<RRwLock<ConstraintState<'a, 'b, C>>>>,
    blocks: AVec<RBasedConstraintBlock<'a, 'b, D::T, C>>,
    ancestry: RArc<ConceptAncestry<'a>>,
    dag_type: PhantomData<D>,
    endpoints: EndpointConfig,
    constraint_explanations: HashMap<AString, (Option<AString>, Option<AString>)>,
    ancestors: HashMap<(Uuid, AString), AVec<AncestorRecord>>,
    topline_constraint_names: LinkedHashSet<AString>,
    _lt_phantom: PhantomData<&'b ()>,
}
impl<'a, 'b, D, C> Driver<'a, 'b, D, C> for RBasedDriver<'a, 'b, D, C>
where
    'a: 'b,
    D: FlowBuilderBase,
    D: FlowBuilderMaterialize<BuilderInputType = RFlowBuilderInput>,
    <D as FlowBuilderBase>::T: ETLFlow<ImportType = RImport, PreambleType = RPreamble> + 'a,
    C: OuterConstraint<'a, 'b, TAncestry = ConceptAncestry<'a>>
        + SatisfiableOuterConstraint<'a, 'b>
        + 'b,
{
    type CB = RBasedConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>;

    fn get_constraint_rwlock(&self, uuid: &(Uuid, AString)) -> RArc<RRwLock<C>> {
        self.constraints.get(uuid).unwrap().clone()
    }
    fn get_preferences(&self) -> AVec<Dialect> {
        vec![
            Dialect::R(R {}),
            Dialect::Python(Python::new(vec![])),
            Dialect::Bash(Bash {}),
        ]
    }

    fn get_endpoints(&'b self) -> &'b EndpointConfig {
        &self.endpoints
    }

    fn get_ancestry(&self) -> RArc<ConceptAncestry<'a>> {
        self.ancestry.clone()
    }
    fn mark_constraint_state_as_satisfied(
        &mut self,
        id: (Uuid, AString),
        state: RArc<RRwLock<ConstraintState<'a, 'b, C>>>,
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
        constraint_block: RBasedConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>,
    ) {
        self.blocks.push(constraint_block);
    }
    fn get_constraint_explanation(
        &self,
        constraint_name: &String,
    ) -> (Option<AString>, Option<AString>) {
        self.constraint_explanations
            .get(constraint_name)
            .unwrap()
            .clone()
    }
    fn get_blocks(&'b self) -> &'b AVec<Self::CB> {
        &self.blocks
    }
    fn get_dependencies(&self) -> AVec<AString> {
        // TODO: add libraries
        vec![]
    }
    fn _new(
        concepts: RArc<RRwLock<HashMap<(Uuid, AString), Concept<'a>>>>,
        constraints: LinkedHashMap<(Uuid, AString), RArc<RRwLock<C>>>,
        ancestry: RArc<ConceptAncestry<'a>>,
        endpoints: EndpointConfig,
        ancestors: HashMap<(Uuid, AString), AVec<AncestorRecord>>,
        topline_constraint_names: LinkedHashSet<AString>,
    ) -> Self {
        Self {
            concepts,
            constraints,
            satisfied_constraints: HashMap::new(),
            blocks: AVec::new(),
            ancestry,
            dag_type: PhantomData,
            endpoints,
            constraint_explanations: <<C as OuterConstraint<'a, 'b>>::TEnum as TConstraintEnum<
                'a,
                'b,
            >>::get_explanations(),
            ancestors,
            topline_constraint_names,
            _lt_phantom: PhantomData,
        }
    }
}
